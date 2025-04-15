use rustecal::{Ecal, EcalComponents};
use rustecal::service::client::ServiceClient;
use rustecal::service::types::ServiceRequest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize eCAL
    Ecal::initialize(Some("echo client rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    // Create a ServiceClient for a service named "echo_service"
    let client = ServiceClient::new("echo_service")?;

    // Create a request with a payload
    let request = ServiceRequest {
        payload: b"Hello from Rust client".to_vec(),
    };

    while Ecal::ok() {
        // Call the "echo" method with a 1000ms timeout
        match client.call("echo", request.clone(), Some(1000)) {
            Some(response) => {
                if response.success {
                    println!(
                        "Response: {}",
                        String::from_utf8_lossy(&response.payload)
                    );
                } else {
                    println!(
                        "Call failed: {}",
                        response.error_msg.unwrap_or("unknown error".into())
                    );
                }
            }
            None => {
                println!("Service call failed or no response received.");
            }
        }
    }

    Ok(())
}
