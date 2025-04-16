use crate::service::client_instance::ClientInstance;
use crate::service::types::{CallState, ServiceRequest, ServiceResponse};
use rustecal_sys::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::ptr;

/// Represents a high-level service client capable of invoking remote procedures.
pub struct ServiceClient {
    pub(crate) handle: *mut eCAL_ServiceClient,
}

impl ServiceClient {
    /// Creates a new `ServiceClient` for the given service name.
    pub fn new(service_name: &str) -> Result<Self, String> {
        let c_service = CString::new(service_name).map_err(|_| "Invalid service name")?;
        let handle = unsafe { eCAL_ServiceClient_New(c_service.as_ptr(), ptr::null(), 0, None) };

        if handle.is_null() {
            Err("Failed to create eCAL_ServiceClient".into())
        } else {
            Ok(Self { handle })
        }
    }

    /// Calls a method and returns the first response received from any server.
    pub fn call(&self, method: &str, request: ServiceRequest, timeout_ms: Option<i32>) -> Option<ServiceResponse> {
        self.call_all(method, request, timeout_ms)?.pop()
    }

    /// Calls a method and returns all responses received from available servers.
    pub fn call_all(
        &self,
        method: &str,
        request: ServiceRequest,
        timeout_ms: Option<i32>,
    ) -> Option<Vec<ServiceResponse>> {
        let c_method = CString::new(method).ok()?;

        let mut response_ptr: *mut eCAL_SServiceResponse = ptr::null_mut();
        let mut response_len: usize = 0;

        let timeout_ptr = timeout_ms
            .as_ref()
            .map(|t| t as *const i32)
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

        let mut responses = Vec::with_capacity(response_len);

        unsafe {
            for i in 0..response_len {
                let item = *response_ptr.add(i);
                let success = CallState::from(item.call_state).is_success();

                let error_msg = if item.error_msg.is_null() {
                    None
                } else {
                    Some(CStr::from_ptr(item.error_msg).to_string_lossy().into_owned())
                };

                let payload = if item.response.is_null() {
                    vec![]
                } else {
                    CStr::from_ptr(item.response as *const i8).to_bytes().to_vec()
                };

                responses.push(ServiceResponse {
                    success,
                    error_msg,
                    payload,
                });
            }

            eCAL_Free(response_ptr as *mut c_void);
        }

        Some(responses)
    }

    /// Returns a list of connected `ClientInstance`s (each one representing a server).
    pub fn get_client_instances(&self) -> Vec<ClientInstance> {
        let mut result = Vec::new();

        unsafe {
            let list_ptr = eCAL_ServiceClient_GetClientInstances(self.handle);
            if list_ptr.is_null() {
                return result;
            }

            let mut offset = 0;
            loop {
                let instance_ptr = *list_ptr.add(offset);
                if instance_ptr.is_null() {
                    break;
                }

                result.push(ClientInstance::from_raw(instance_ptr));
                offset += 1;
            }

            eCAL_ClientInstances_Delete(list_ptr);
        }

        result
    }
}

impl Drop for ServiceClient {
    fn drop(&mut self) {
        unsafe {
            eCAL_ServiceClient_Delete(self.handle);
        }
    }
}
