use rustecal::{Ecal, EcalComponents};
use rustecal::service::server::ServiceServer;
use rustecal::service::types::{ServiceRequest, ServiceResponse, MethodInfo, ServiceId};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize eCAL
    Ecal::initialize(Some("echo server rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    // Create the service server
    let mut server = ServiceServer::new("echo_service")?;

    // Register the "echo" method
    server.add_method("echo", Box::new(|info: MethodInfo, req: ServiceRequest| {
        println!(
            "Received call on method '{}', payload: {:?}",
            info.method_name,
            String::from_utf8_lossy(&req.payload)
        );

        ServiceResponse {
            success: true,
            server_id: ServiceId {
                service_id: unsafe { std::mem::zeroed() }, // Placeholder until response is filled by core
            },
            payload: req.payload,
            error_msg: None,
        }
    }))?;

    println!("Echo service running. Press Ctrl+C to exit.");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
