//! Shared service-related data structures for client and server.

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
    pub error_message: Option<String>,
    /// Raw byte buffer containing the serialized response.
    pub payload: Vec<u8>,
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
