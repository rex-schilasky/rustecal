use crate::service::types::{ServiceId, ServiceRequest, ServiceResponse};
use rustecal_sys::*;
use std::ffi::CString;
use std::os::raw::c_void;

/// Represents a connection to a specific service server instance.
#[derive(Debug)]
pub struct ClientInstance {
    pub(crate) client_handle: *mut eCAL_ServiceClient,
    pub(crate) instance: eCAL_SServiceId,
}

impl ClientInstance {
    pub fn from_ffi(client_handle: *mut eCAL_ServiceClient, raw: &eCAL_SServiceId) -> Self {
        Self {
            client_handle,
            instance: *raw,
        }
    }

    pub fn id(&self) -> ServiceId {
        unsafe { ServiceId::from_ffi(&self.instance) }
    }

    pub fn call(&self, method: &str, request: ServiceRequest, timeout_ms: Option<i32>) -> Option<ServiceResponse> {
        let c_method = CString::new(method).ok()?;
        let timeout_ptr = timeout_ms
            .as_ref()
            .map(|t| t as *const i32)
            .unwrap_or(std::ptr::null());

        let response_ptr = unsafe {
            eCAL_ClientInstance_CallWithResponse(
                self.client_handle as *mut eCAL_ClientInstance,
                c_method.as_ptr(),
                request.payload.as_ptr() as *const c_void,
                request.payload.len(),
                timeout_ptr,
            )
        };

        if response_ptr.is_null() {
            return Some(ServiceResponse {
                success: false,
                error_msg: Some("call failed".into()),
                payload: vec![],
            });
        }

        Some(unsafe { ServiceResponse::from_raw_response(response_ptr) })
    }
}
