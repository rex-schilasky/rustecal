use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_bytes::BytesMessage;

fn main() {
    Ecal::initialize(Some("bytes subscriber rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let mut subscriber = TypedSubscriber::<BytesMessage>::new("bytes")
        .expect("Failed to create subscriber");

    subscriber.set_callback(|msg: BytesMessage| {
        println!("Received: {:?}", String::from_utf8_lossy(&msg.0));
    });

    println!("Waiting for byte messages...");

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}
