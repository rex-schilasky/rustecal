use rustecal::{Ecal, EcalComponents, TypedPublisher};

fn main() {
    Ecal::initialize(Some("minimal string publisher rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let publisher = TypedPublisher::<String>::new("hello")
        .expect("Failed to create publisher");

    let mut cnt = 0;
    while Ecal::ok() {
        cnt += 1;
        let msg = format!("HELLO WORLD FROM RUST ({})", cnt);
        publisher.send(&msg);
        println!("Sent: {}", msg);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}