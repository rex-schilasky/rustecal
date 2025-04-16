//! Shared service-related data structures for client and server.

use rustecal_sys::*;

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
    /// Construct from raw FFI pointer to `eCAL_SServiceResponse` (used in client).
    pub unsafe fn from_raw_response(ptr: *mut eCAL_SServiceResponse) -> Self {
        if ptr.is_null() {
            return Self {
                success: false,
                error_msg: Some("null response".into()),
                payload: vec![],
            };
        }

        let response = unsafe { *ptr };

        let payload = unsafe {
            std::slice::from_raw_parts(
                response.response as *const u8,
                response.response_length,
            )
        }
            .to_vec();

        let call_state = CallState::from(response.call_state);

        Self {
            success: call_state.is_success(),
            error_msg: if call_state.is_success() {
                None
            } else {
                Some(format!("{call_state:?}"))
            },
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
