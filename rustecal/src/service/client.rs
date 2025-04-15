// src/service/client.rs

use crate::service::types::{CallState, ServiceRequest, ServiceResponse};
use crate::service::client_instance::ClientInstance;
use rustecal_sys::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::ptr;

/// A high-level abstraction representing a service client that can call all available service servers.
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

    /// Calls a method and returns the first response (if any).
    pub fn call(&self, method: &str, request: ServiceRequest, timeout_ms: Option<i32>) -> Option<ServiceResponse> {
        let mut responses = self.call_all(method, request, timeout_ms)?;
        responses.pop()
    }

    /// Calls a method and returns all responses received from servers.
    pub fn call_all(&self, method: &str, request: ServiceRequest, timeout_ms: Option<i32>) -> Option<Vec<ServiceResponse>> {
        let c_method = CString::new(method).ok()?;
        let mut response_ptr: *mut eCAL_SServiceResponse = ptr::null_mut();
        let mut response_len: usize = 0;

        let timeout_ref: Option<i32> = timeout_ms;
        let timeout_ptr = timeout_ref
            .as_ref()
            .map(|v| v as *const i32)
            .unwrap_or(ptr::null());

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

        let mut responses = Vec::new();

        unsafe {
            for i in 0..response_len {
                let item = *response_ptr.add(i);
                let success = CallState::from(item.call_state).is_success();

                let error_msg = if item.error_msg.is_null() {
                    None
                } else {
                    Some(CStr::from_ptr(item.error_msg).to_string_lossy().into_owned())
                };

                // FIXME: Intermediate fix: fall back to null-terminated string
                //        In future, use item.response_length (currently unreliable)
                let payload = if item.response.is_null() {
                    vec![]
                } else {
                    CStr::from_ptr(item.response as *const i8).to_bytes().to_vec()
                };

                responses.push(ServiceResponse {
                    success,
                    payload,
                    error_msg,
                });
            }

            eCAL_Free(response_ptr as *mut c_void);
        }

        Some(responses)
    }

    /// Returns a list of connected `ClientInstance`s.
    pub fn get_client_instances(&self) -> Vec<ClientInstance> {
        // TODO: Implement this in the next step using eCAL_GetServiceClientInstances
        vec![]
    }
}

impl Drop for ServiceClient {
    fn drop(&mut self) {
        unsafe {
            eCAL_ServiceClient_Delete(self.handle);
        }
    }
}
