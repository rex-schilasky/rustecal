use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct ServiceRequest {
    pub method: String,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ServiceResponse {
    pub success: bool,
    pub payload: Vec<u8>,
}

pub type ServiceCallback = Box<dyn Fn(ServiceRequest) -> ServiceResponse + Send + Sync + 'static>;
pub type SharedServiceCallback = Arc<Mutex<Option<ServiceCallback>>>;
