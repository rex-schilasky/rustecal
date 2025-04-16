//! Types used by the service layer of eCAL.

use crate::ecal::types::{DataTypeInfo, EntityId};
use rustecal_sys::*;
use std::ffi::CStr;

/// Metadata about a method, including name and (optional) type info.
#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub method_name: String,
    pub request_type: Option<String>,
    pub response_type: Option<String>,
}

/// Represents a serialized request to a service.
#[derive(Debug, Clone)]
pub struct ServiceRequest {
    pub payload: Vec<u8>,
}

/// Represents a serialized response from a service.
#[derive(Debug, Clone)]
pub struct ServiceResponse {
    pub success: bool,
    pub payload: Vec<u8>,
    pub error_msg: Option<String>,
}

/// Enum representing the result of a service call.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CallState {
    None,
    Executed,
    Timeouted,
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
            0 => CallState::None,
            1 => CallState::Executed,
            2 => CallState::Timeouted,
            3 => CallState::Failed,
            other => CallState::Unknown(other),
        }
    }
}

/// Callback type used by service servers for responding to method calls.
pub type ServiceCallback = Box<dyn Fn(MethodInfo, ServiceRequest) -> ServiceResponse + Send + Sync + 'static>;

/// A unique identifier for a service.
#[derive(Debug, Clone)]
pub struct ServiceId {
    pub service_id: EntityId,
    pub service_name: Option<String>,
}

impl ServiceId {
    pub unsafe fn from_ffi(raw: &eCAL_SServiceId) -> Self {
        ServiceId {
            service_id: EntityId::from(raw.service_id),
            service_name: if raw.service_name.is_null() {
                None
            } else {
                Some(CStr::from_ptr(raw.service_name).to_string_lossy().into_owned())
            },
        }
    }
}
