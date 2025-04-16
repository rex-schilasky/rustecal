use crate::service::client_instance::ClientInstance;
use crate::service::types::{ServiceRequest, ServiceResponse};
use rustecal_sys::*;
use std::ffi::CString;
use std::os::raw::c_void;

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

        let mut response_ptr: *mut eCAL_SServiceResponse = std::ptr::null_mut();
        let mut response_size: usize = 0;
        let timeout_ptr = timeout_ms
            .as_ref()
            .map(|t| t as *const i32)
            .unwrap_or(std::ptr::null());

        let result = unsafe {
            eCAL_ServiceClient_CallWithResponse(
                self.handle,
                c_method.as_ptr(),
                request.payload.as_ptr() as *const c_void,
                request.payload.len(),
                &mut response_ptr,
                &mut response_size,
                timeout_ptr,
            )
        };

        if result == 0 || response_ptr.is_null() {
            return None;
        }

        Some(unsafe { ServiceResponse::from_raw_response(response_ptr) })
    }

    // TODO: Implement instance query when correct FFI API becomes available.
    pub fn client_instances(&self) -> Vec<ClientInstance> {
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
