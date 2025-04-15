// src/service/client_instance.rs

use crate::service::types::{CallState, ServiceRequest, ServiceResponse};
use rustecal_sys::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::ptr;

/// Represents a connection to a specific service server instance.
pub struct ClientInstance {
    pub(crate) handle: *mut eCAL_ClientInstance,
}

impl ClientInstance {
    /// Calls a method on this specific service instance and returns a response.
    ///
    /// # Arguments
    /// * `method` - The name of the method to call.
    /// * `request` - The request payload.
    /// * `timeout_ms` - Optional timeout in milliseconds.
    pub fn call(
        &self,
        method: &str,
        request: ServiceRequest,
        timeout_ms: Option<i32>,
    ) -> Option<ServiceResponse> {
        let c_method = CString::new(method).ok()?;

        let timeout_ptr = timeout_ms
            .as_ref()
            .map(|v| v as *const i32)
            .unwrap_or(ptr::null());

        let response_ptr = unsafe {
            eCAL_ClientInstance_CallWithResponse(
                self.handle,
                c_method.as_ptr(),
                request.payload.as_ptr() as *const c_void,
                request.payload.len(),
                timeout_ptr,
            )
        };

        if response_ptr.is_null() {
            return None;
        }

        let response = unsafe { &*response_ptr };

        let success = CallState::from(response.call_state).is_success();

        let error_msg = if response.error_msg.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(response.error_msg) }.to_string_lossy().into_owned())
        };

        // FIXME: This is an intermediate workaround.
        // Properly interpret response_length once it is set reliably.
        let payload = if response.response.is_null() {
            vec![]
        } else {
            unsafe { CStr::from_ptr(response.response as *const i8).to_bytes().to_vec() }
        };

        unsafe {
            eCAL_Free(response_ptr as *mut c_void);
        }

        Some(ServiceResponse {
            success,
            payload,
            error_msg,
        })
    }
}
