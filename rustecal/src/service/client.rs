use crate::service::types::{ServiceRequest, ServiceResponse};

/// Represents a client that can call an eCAL service.
pub struct ServiceClient {
    service_name: String,
}

impl ServiceClient {
    /// Creates a new `ServiceClient` with the given service name.
    pub fn new(service_name: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
        }
    }

    /// Calls the service with a given request.
    /// In a real implementation, this would communicate with an eCAL server.
    pub fn call_service(&self, request: ServiceRequest) -> Option<ServiceResponse> {
        println!(
            "Calling service '{}' with request: {:?}",
            self.service_name, request
        );
    
        // Simulate a dummy response: echo the payload
        Some(ServiceResponse {
            success: true,
            payload: request.payload.clone(),
        })
    }
}
