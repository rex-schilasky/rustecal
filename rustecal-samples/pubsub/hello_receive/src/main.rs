use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_string::StringMessage;

fn main() {
    Ecal::initialize(Some("hello string subscriber rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let mut subscriber = TypedSubscriber::<StringMessage>::new("hello")
        .expect("Failed to create subscriber");

    subscriber.set_callback(|msg| {
        println!("Received: {}", msg.0);
    });

    println!("Waiting for messages on topic 'hello'...");

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}