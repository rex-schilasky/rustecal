use rustecal::{Ecal, EcalComponents};
use rustecal::service::client::ServiceClient;
use rustecal::service::types::ServiceRequest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize eCAL
    Ecal::initialize(Some("echo client rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    // Create the client for "echo_service"
    let client = ServiceClient::new("echo_service")?;

    // Create the request payload
    let request = ServiceRequest {
        payload: b"Hello from echo client rust".to_vec(),
    };

    while Ecal::ok() {
        // Call the method and retrieve a single response (if any)
        match client.call("echo", request.clone(), Some(1000)) {
            Some(response) => {
                if response.success {
                    println!(
                        "Response: {}",
                        String::from_utf8_lossy(&response.payload)
                    );
                } else {
                    println!(
                        "Response failed: {}",
                        response.error_msg.as_deref().unwrap_or("Unknown error")
                    );
                }
            }
            None => {
                println!("No response received or call failed.");
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ok(())
}
