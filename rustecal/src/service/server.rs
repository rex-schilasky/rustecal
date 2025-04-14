use crate::service::types::{ServiceCallback, SharedServiceCallback, ServiceRequest, ServiceResponse};
use std::sync::{Arc, Mutex};

/// Main struct for a service server.
pub struct ServiceServer {
    service_name: String,
    callback: SharedServiceCallback,
}

impl ServiceServer {
    pub fn new(service_name: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
            callback: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(ServiceRequest) -> ServiceResponse + Send + Sync + 'static,
    {
        *self.callback.lock().unwrap() = Some(Box::new(callback));
    }

    pub fn start(&self) {
        println!("Starting service '{}'", self.service_name);
        // Placeholder for actual eCAL server integration
    }
}
