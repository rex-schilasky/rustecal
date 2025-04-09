use rustecal::{Ecal, EcalComponents, TypedSubscriber};

fn main() {
    Ecal::initialize(Some("hello string subscriber rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let mut subscriber = TypedSubscriber::<String>::new("hello")
        .expect("Failed to create subscriber");

    subscriber.set_callback(|msg| {
        println!("Received: {}", msg);
    });

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}
