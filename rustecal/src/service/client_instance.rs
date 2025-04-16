use crate::service::types::{ServiceRequest, ServiceResponse};
use rustecal_sys::*;
use std::ffi::{CString, CStr};
use std::os::raw::c_void;

/// Represents a connection to a specific service server instance.
#[derive(Debug)]
pub struct ClientInstance {
    pub(crate) instance: *mut eCAL_ClientInstance,
}

impl ClientInstance {
    /// Constructs a `ClientInstance` from a raw pointer returned by eCAL.
    pub fn from_raw(raw: *mut eCAL_ClientInstance) -> Self {
        Self {
            instance: raw,
        }
    }

    /// Calls a method on this specific service instance.
    pub fn call(
        &self,
        method: &str,
        request: ServiceRequest,
        timeout_ms: Option<i32>,
    ) -> Option<ServiceResponse> {
        let c_method = CString::new(method).ok()?;
        let timeout_ptr = timeout_ms
            .as_ref()
            .map(|t| t as *const i32)
            .unwrap_or(std::ptr::null());

        let response_ptr = unsafe {
            eCAL_ClientInstance_CallWithResponse(
                self.instance,
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

        unsafe {
            let response = *response_ptr;

            let success = crate::service::types::CallState::from(response.call_state).is_success();

            let error_msg = if response.error_msg.is_null() {
                None
            } else {
                Some(CStr::from_ptr(response.error_msg).to_string_lossy().into_owned())
            };

            let payload = if response.response.is_null() {
                vec![]
            } else {
                CStr::from_ptr(response.response as *const i8)
                    .to_bytes()
                    .to_vec()
            };

            eCAL_Free(response_ptr as *mut c_void);

            Some(ServiceResponse {
                success,
                error_msg,
                payload,
            })
        }
    }
}
