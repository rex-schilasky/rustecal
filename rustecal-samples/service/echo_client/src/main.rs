use rustecal::service::client::ServiceClient;
use rustecal::service::types::ServiceRequest;

fn main() {
    // Create a ServiceClient for a service named "echo_service"
    let client = ServiceClient::new("echo_service");

    // Create a request
    let request = ServiceRequest {
        method: "echo".to_string(),
        payload: b"Hello from client".to_vec(),
    };

    // Call the service
    match client.call_service(request) {
        Some(response) => {
            println!("Received response: {:?}", String::from_utf8_lossy(&response.payload));
        }
        None => {
            println!("Service call failed or returned no response.");
        }
    }
}
