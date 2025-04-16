use rustecal_sys::*;
use std::ffi::CStr;

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

#[derive(Debug, Clone)]
pub struct ServiceRequest {
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ServiceResponse {
    pub success: bool,
    pub error_msg: Option<String>,
    pub payload: Vec<u8>,
}

impl ServiceResponse {
    /// Constructs from an `eCAL_SServiceResponse` struct.
    ///
    /// ⚠️ This version ignores `response_length` due to known issues in the C API,
    /// and instead assumes the `response` pointer is a null-terminated string.
    ///
    /// ⚠️ This is not safe for binary payloads or strings containing embedded `\0`,
    /// but avoids crashes due to uninitialized or garbage `response_length` values.
    ///
    /// ✅ To support full binary buffers in the future, switch back to using `response_length`.
    pub fn from_struct(response: &eCAL_SServiceResponse) -> Self {
        let success = CallState::from(response.call_state).is_success();

        let error_msg = if response.error_msg.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(response.error_msg).to_string_lossy().into_owned()
            })
        };

        let payload = if response.response.is_null() {
            vec![]
        } else {
            unsafe {
                CStr::from_ptr(response.response as *const i8)
                    .to_bytes()
                    .to_vec()
            }
        };

        Self {
            success,
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
