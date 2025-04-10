use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_string::StringMessage;
use rustecal::pubsub::typed_subscriber::Received;

fn main() {
    Ecal::initialize(Some("hello string subscriber rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let mut subscriber = TypedSubscriber::<StringMessage>::new("hello")
        .expect("Failed to create subscriber");

    subscriber.set_callback(|msg: Received<StringMessage>| {
        println!("Received: {}", msg.msg.0);
        println!("Time    : {}", msg.timestamp);
        println!("Clock   : {}", msg.clock);
        println!();
    });

    println!("Waiting for messages on topic 'hello'...");

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ecal::finalize();
}
