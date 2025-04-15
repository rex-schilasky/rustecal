use crate::service::types::{CallState, ServiceRequest, ServiceResponse};
use rustecal_sys::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::ptr;

/// Represents a client that can call an eCAL service.
pub struct ServiceClient {
    pub(crate) handle: *mut eCAL_ServiceClient,
}

impl ServiceClient {
    pub fn new(service_name: &str) -> Result<Self, String> {
        let c_service = CString::new(service_name).map_err(|_| "Invalid service name")?;

        let handle = unsafe { eCAL_ServiceClient_New(c_service.as_ptr(), std::ptr::null(), 0, None) };

        if handle.is_null() {
            Err("Failed to create eCAL_ServiceClient".into())
        } else {
            Ok(Self { handle })
        }
    }

    pub fn call(&self, method: &str, request: ServiceRequest, timeout_ms: Option<i32>) -> Option<ServiceResponse> {
        let c_method = CString::new(method).ok()?;
        let mut response_ptr: *mut eCAL_SServiceResponse = ptr::null_mut();
        let mut response_len: usize = 0;

        let timeout_ptr = match timeout_ms {
            Some(ref value) => value as *const i32,
            None => ptr::null(),
        };

        let result = unsafe {
            eCAL_ServiceClient_CallWithResponse(
                self.handle,
                c_method.as_ptr(),
                request.payload.as_ptr() as *const c_void,
                request.payload.len(),
                &mut response_ptr,
                &mut response_len,
                timeout_ptr,
            )
        };

        if result != 0 || response_ptr.is_null() || response_len == 0 {
            return None;
        }

        let response = unsafe { *response_ptr };

        let success = CallState::from(response.call_state).is_success();

        let error_message = if response.error_msg.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(response.error_msg) }.to_string_lossy().into_owned())
        };

        let payload = unsafe {
            std::slice::from_raw_parts(
                response.response as *const u8,
                response.response_length,
            ).to_vec()
        };

        unsafe {
            eCAL_Free(response_ptr as *mut c_void);
        }

        Some(ServiceResponse {
            success,
            error_msg: error_message,
            payload,
        })
    }
}

impl Drop for ServiceClient {
    fn drop(&mut self) {
        unsafe {
            eCAL_ServiceClient_Delete(self.handle);
        }
    }
}
