use rustecal::{Ecal, EcalComponents};
use rustecal::service::server::ServiceServer;
use rustecal::service::types::{MethodInfo};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize eCAL
    Ecal::initialize(Some("echo server rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    // Create the service server
    let mut server = ServiceServer::new("echo_service")?;

    // Register the "echo" method using the simplified API
    server.add_method("echo", Box::new(|info: MethodInfo, req: &[u8]| {
        println!(
            "Received call on method '{}', payload: {}",
            info.method_name,
            String::from_utf8_lossy(req)
        );

        // Echo the payload back
        req.to_vec()
    }))?;

    println!("Echo service running. Press Ctrl+C to exit.");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
