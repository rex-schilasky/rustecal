use rustecal_sys::*;
use crate::service::types::{MethodInfo, ServiceCallback, ServiceRequest};
use std::ffi::{CStr, CString};
use std::os::raw::{c_void, c_char};
use std::ptr;
use std::sync::{Arc, Mutex};

/// A handle to an eCAL service server.
pub struct ServiceServer {
    pub(crate) handle: *mut eCAL_ServiceServer,
    callback: Arc<Mutex<Option<ServiceCallback>>>,
}

impl ServiceServer {
    /// Creates a new `ServiceServer` with a given service name.
    pub fn new(service_name: &str) -> Result<Self, String> {
        let c_name = CString::new(service_name).map_err(|_| "Invalid service name")?;

        let handle = unsafe {
            eCAL_ServiceServer_New(c_name.as_ptr(), None)
        };

        if handle.is_null() {
            return Err("Failed to create eCAL_ServiceServer".into());
        }

        Ok(Self {
            handle,
            callback: Arc::new(Mutex::new(None)),
        })
    }

    /// Registers a method with a Rust callback.
    pub fn add_method(&mut self, method: &str, callback: ServiceCallback) -> Result<(), String> {
        let c_method = CString::new(method).map_err(|_| "Invalid method name")?;
        let method_info = eCAL_SServiceMethodInformation {
            method_name: c_method.as_ptr(),
            request_type: eCAL_SDataTypeInformation {
                encoding: ptr::null(),
                name: ptr::null(),
                descriptor: ptr::null(),
                descriptor_length: 0,
            },
            response_type: eCAL_SDataTypeInformation {
                encoding: ptr::null(),
                name: ptr::null(),
                descriptor: ptr::null(),
                descriptor_length: 0,
            },
        };        

        let user_data = Arc::into_raw(Arc::clone(&self.callback)) as *mut c_void;

        let result = unsafe {
            eCAL_ServiceServer_SetMethodCallback(
                self.handle,
                &method_info,
                Some(service_callback_trampoline),
                user_data,
            )
        };

        if result != 0 {
            Err("Failed to register method callback".into())
        } else {
            *self.callback.lock().unwrap() = Some(callback);
            Ok(())
        }
    }
}

impl Drop for ServiceServer {
    fn drop(&mut self) {
        unsafe {
            eCAL_ServiceServer_Delete(self.handle);
        }
    }
}

unsafe extern "C" fn service_callback_trampoline(
    method_info: *const eCAL_SServiceMethodInformation,
    request: *const c_void,
    request_len: usize,
    response: *mut *mut c_void,
    response_len: *mut usize,
    user_data: *mut c_void,
) -> i32 {
    if method_info.is_null() || request.is_null() || user_data.is_null() {
        return 0;
    }

    let callback = unsafe {
        Arc::from_raw(user_data as *const Mutex<Option<ServiceCallback>>)
    };

    let method_name = unsafe {
        cstr_opt((*method_info).method_name).unwrap_or_default()
    };

    let request_type = unsafe {
        cstr_opt((*method_info).request_type.name)
    };

    let response_type = unsafe {
        cstr_opt((*method_info).response_type.name)
    };

    let request_payload = unsafe {
        std::slice::from_raw_parts(request as *const u8, request_len).to_vec()
    };

    let method_info = MethodInfo {
        method_name,
        request_type,
        response_type,
    };

    let request = ServiceRequest {
        payload: request_payload,
    };

    let result = callback.lock().unwrap().as_ref().map(|cb| cb(method_info, request));

    if let Some(resp) = result {
        let len = resp.payload.len();
        if len > 0 {
            let buf = unsafe {
                libc::malloc(len) as *mut u8
            };
            if !buf.is_null() {
                unsafe {
                    std::ptr::copy_nonoverlapping(resp.payload.as_ptr(), buf, len);
                    *response = buf as *mut c_void;
                    *response_len = len;
                }
                std::mem::forget(callback);
                return 1;
            }
        }
    }

    std::mem::forget(callback);
    0
}

fn cstr_opt(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() })
    }
}
