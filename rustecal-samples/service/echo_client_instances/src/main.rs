use rustecal::{Ecal, EcalComponents};
use rustecal::service::client::ServiceClient;
use rustecal::service::types::ServiceRequest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize eCAL
    Ecal::initialize(Some("echo client rust using instances"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    // Create the client for "echo_service"
    let client = ServiceClient::new("echo_service")?;

    // Create the request payload
    let request = ServiceRequest {
        payload: b"Hello from echo client rust".to_vec(),
    };

    while Ecal::ok() {
        let instances = client.get_client_instances();

        if instances.is_empty() {
            println!("No service instances available.");
        } else {
            for (i, instance) in instances.iter().enumerate() {
                match instance.call("echo", request.clone(), Some(1000)) {
                    Some(response) => {
                        if response.success {
                            println!(
                                "Instance #{} response: {}",
                                i,
                                String::from_utf8_lossy(&response.payload)
                            );
                        } else {
                            println!(
                                "Instance #{} failed: {}",
                                i,
                                response.error_msg.as_deref().unwrap_or("Unknown error")
                            );
                        }
                    }
                    None => {
                        println!("Instance #{}: No response or call failed.", i);
                    }
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    Ok(())
}
