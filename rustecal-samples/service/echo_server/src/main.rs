use rustecal::service::server::ServiceServer;
use rustecal::service::types::{ServiceRequest, ServiceResponse};

fn main() {
    // Create a new service server with name "echo_service"
    let mut server = ServiceServer::new("echo_service");

    // Set the callback to handle incoming requests
    server.set_callback(|request: ServiceRequest| -> ServiceResponse {
        println!("Received request: {:?}", String::from_utf8_lossy(&request.payload));

        // Echo back the request payload with a prefix
        let mut response_payload = b"Echo: ".to_vec();
        response_payload.extend(request.payload);

        ServiceResponse {
            success: true,
            payload: response_payload,
        }
    });

    println!("Service server is running. Press Ctrl+C to exit...");

    // Keep the server alive
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
