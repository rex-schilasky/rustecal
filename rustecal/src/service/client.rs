use rustecal_sys::*;
use crate::service::types::{ServiceRequest, ServiceResponse};
use std::ffi::{CStr, CString};
use std::os::raw::{c_void};

/// A handle to a specific method of a remote eCAL service.
pub struct ServiceClient {
    client: *mut eCAL_ServiceClient,
    method_name: CString,
}

impl ServiceClient {
    /// Creates a new `ServiceClient` for the specified service and method.
    pub fn new(service_name: &str, method_name: &str) -> Result<Self, String> {
        let c_service = CString::new(service_name).map_err(|_| "Invalid service name")?;
        let c_method = CString::new(method_name).map_err(|_| "Invalid method name")?;

        let method_info = eCAL_SServiceMethodInformation {
            method_name: c_method.as_ptr(),
            request_type: eCAL_SDataTypeInformation {
                encoding: std::ptr::null(),
                name: std::ptr::null(),
                descriptor: std::ptr::null(),
                descriptor_length: 0,
            },
            response_type: eCAL_SDataTypeInformation {
                encoding: std::ptr::null(),
                name: std::ptr::null(),
                descriptor: std::ptr::null(),
                descriptor_length: 0,
            },
        };

        let client = unsafe {
            eCAL_ServiceClient_New(
                c_service.as_ptr(),
                &method_info,
                1,
                None, // No event callback
            )
        };

        if client.is_null() {
            return Err("Failed to create eCAL_ServiceClient".into());
        }

        Ok(Self {
            client,
            method_name: c_method,
        })
    }

    /// Calls the remote method and returns a `ServiceResponse`.
    pub fn call(&self, request: ServiceRequest, timeout_ms: i32) -> Option<ServiceResponse> {
        let mut resp_ptr: *mut eCAL_SServiceResponse = std::ptr::null_mut();
        let mut resp_len: usize = 0;

        let result = unsafe {
            eCAL_ServiceClient_CallWithResponse(
                self.client,
                self.method_name.as_ptr(),
                request.payload.as_ptr() as *const c_void,
                request.payload.len(),
                &mut resp_ptr,
                &mut resp_len,
                &timeout_ms,
            )
        };

        if result == 0 || resp_ptr.is_null() || resp_len == 0 {
            return None;
        }

        let responses = unsafe { std::slice::from_raw_parts(resp_ptr, resp_len) };

        let response = responses.first().map(|resp| {
            let error_msg = unsafe {
                if resp.error_msg.is_null() {
                    None
                } else {
                    Some(CStr::from_ptr(resp.error_msg).to_string_lossy().into_owned())
                }
            };

            let payload = unsafe {
                std::slice::from_raw_parts(resp.response as *const u8, resp.response_length).to_vec()
            };

            ServiceResponse {
                success: resp.ret_state == 0,
                error_message: error_msg,
                payload,
            }
        });

        unsafe {
            eCAL_Free(resp_ptr as *mut c_void);
        }

        response
    }
}

impl Drop for ServiceClient {
    fn drop(&mut self) {
        unsafe {
            eCAL_ServiceClient_Delete(self.client);
        }
    }
}
