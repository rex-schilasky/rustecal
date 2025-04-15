use rustecal::{Ecal, EcalComponents};
use rustecal::service::client::ServiceClient;
use rustecal::service::types::ServiceRequest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize eCAL
    Ecal::initialize(Some("echo client rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    // Create the service client for the "echo" method on "echo_service"
    let client = ServiceClient::new("echo_service", "echo")?;

    // Prepare a service request
    let request = ServiceRequest {
        payload: b"Hello from Rust client".to_vec(),
    };

    while Ecal::ok() {
        // Call the service with a timeout of 1000 milliseconds (1 second)
        match client.call(request.clone(), 1000) {
            Some(response) => {
                if response.success {
                    println!(
                        "Received echo response: {:?}",
                        String::from_utf8_lossy(&response.payload)
                    );
                } else {
                    println!(
                        "Service responded with error: {:?}",
                        response.error_message.unwrap_or("Unknown error".to_string())
                    );
                }
            }
            None => {
                println!("Service call failed or no response received.");
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ok(())
}
