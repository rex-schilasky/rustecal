use rustecal::{Ecal, EcalComponents, TypedSubscriber};

fn main() {
    Ecal::initialize(Some("minimal string subscriber rust"), EcalComponents::DEFAULT).unwrap();

    let mut sub = TypedSubscriber::<String>::new("hello").expect("Failed to create subscriber");
    sub.set_callback(|msg| {
        println!("Received: {}", msg);
    });

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}
