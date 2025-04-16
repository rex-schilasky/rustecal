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

#[derive(Debug, Clone)]
pub struct ServiceRequest {
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ServiceResponse {
    pub success: bool,
    pub server_id: ServiceId,
    pub error_msg: Option<String>,
    pub payload: Vec<u8>,
}

impl ServiceResponse {
    pub fn from_struct(response: &eCAL_SServiceResponse) -> Self {
        let success = CallState::from(response.call_state).is_success();

        let server_id = unsafe { ServiceId::from_ffi(&response.server_id) };

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
            server_id,
            error_msg,
            payload,
        }
    }
}

/// Metadata passed to method callbacks about the method interface.
#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub method_name: String,
    pub request_type: Option<String>,
    pub response_type: Option<String>,
}

/// The service callback signature used by ServiceServer.
pub type ServiceCallback = Box<dyn Fn(MethodInfo, ServiceRequest) -> ServiceResponse + Send + Sync + 'static>;
