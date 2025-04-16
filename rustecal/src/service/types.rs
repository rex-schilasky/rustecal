//! Shared service-related data structures for client and server.

use rustecal_sys::*;
use std::ffi::CStr;

/// Enum representing the result of a service call.
#[derive(Debug, Clone, Copy)]
pub enum CallState {
    None,
    Executed,
    Timeout,
    Failed,
    Unknown(i32),
}

impl CallState {
    pub fn is_success(&self) -> bool {
        matches!(self, CallState::Executed)
    }
}

impl From<i32> for CallState {
    fn from(value: i32) -> Self {
        match value {
            x if x == rustecal_sys::eCAL_eCallState_eCAL_eCallState_none => CallState::None,
            x if x == rustecal_sys::eCAL_eCallState_eCAL_eCallState_executed => CallState::Executed,
            x if x == rustecal_sys::eCAL_eCallState_eCAL_eCallState_timeouted => CallState::Timeout,
            x if x == rustecal_sys::eCAL_eCallState_eCAL_eCallState_failed => CallState::Failed,
            other => CallState::Unknown(other),
        }
    }
}

/// A service request as passed to or received from eCAL service callbacks.
#[derive(Debug, Clone)]
pub struct ServiceRequest {
    /// Raw byte buffer of the serialized request.
    pub payload: Vec<u8>,
}

/// A service response to return from the service handler.
#[derive(Debug, Clone)]
pub struct ServiceResponse {
    /// Indicates whether the service call was successful.
    pub success: bool,
    /// Optional error message (usually empty if success = true).
    pub error_msg: Option<String>,
    /// Raw byte buffer containing the serialized response.
    pub payload: Vec<u8>,
}

impl ServiceResponse {
    /// Constructs a `ServiceResponse` from a raw FFI pointer returned by eCAL.
    ///
    /// ⚠️ Due to a known issue in the eCAL C API, the `response_length` field of
    /// `eCAL_SServiceResponse` may be **uninitialized** and contain garbage values,
    /// even if the `response` pointer is valid and contains a proper string.
    ///
    /// To avoid undefined behavior, we **ignore `response_length` entirely** for now
    /// and fallback to assuming that the response is a null-terminated C string.
    ///
    /// When the C API is fixed to reliably populate `response_length`, this function
    /// should be updated to use a proper byte slice (see commented code below).
    pub unsafe fn from_raw_response(ptr: *mut eCAL_SServiceResponse) -> Self {
        if ptr.is_null() {
            return Self {
                success: false,
                error_msg: Some("null response".into()),
                payload: vec![],
            };
        }

        let response = unsafe { *ptr };

        // Workaround: use null-terminated parsing only
        let payload = if response.response.is_null() {
            vec![]
        } else {
            unsafe {
                CStr::from_ptr(response.response as *const i8)
                    .to_bytes()
                    .to_vec()
            }
        };

        // ✅ Future-safe version (uncomment when C API bug is resolved)
        /*
        let payload = if response.response.is_null() || response.response_length == 0 {
            vec![]
        } else {
            unsafe {
                std::slice::from_raw_parts(
                    response.response as *const u8,
                    response.response_length,
                )
                .to_vec()
            }
        };
        */

        let call_state = CallState::from(response.call_state);

        let error_msg = if response.error_msg.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(response.error_msg)
                    .to_string_lossy()
                    .into_owned()
            })
        };

        Self {
            success: call_state.is_success(),
            error_msg,
            payload,
        }
    }
}

/// Metadata passed to method callbacks about the method interface.
#[derive(Debug, Clone)]
pub struct MethodInfo {
    /// Method name (e.g. "add", "multiply").
    pub method_name: String,
    /// Optional type name of request (e.g. "MyRequest").
    pub request_type: Option<String>,
    /// Optional type name of response (e.g. "MyResponse").
    pub response_type: Option<String>,
}

/// Callback type used for handling service method invocations.
///
/// This is a boxed Rust function or closure that receives method info and request,
/// and returns a service response.
pub type ServiceCallback = Box<dyn Fn(MethodInfo, ServiceRequest) -> ServiceResponse + Send + Sync + 'static>;

/// A unique identifier for a service instance (for matching / addressing).
#[derive(Debug, Clone, Copy)]
pub struct ServiceId {
    pub service_id: eCAL_SEntityId,
}

impl ServiceId {
    pub unsafe fn from_ffi(raw: &eCAL_SServiceId) -> Self {
        Self {
            service_id: raw.service_id,
        }
    }
}
