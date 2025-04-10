use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_bytes::BytesMessage;
use rustecal::pubsub::typed_subscriber::Received;

fn main() {
    Ecal::initialize(Some("bytes subscriber rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let mut subscriber = TypedSubscriber::<BytesMessage>::new("blob")
        .expect("Failed to create subscriber");

    subscriber.set_callback(|msg: Received<BytesMessage>| {
        let buffer = &msg.msg.0;
        if buffer.is_empty() {
            return;
        }

        let content = buffer[0] as u8;

        println!("----------------------------------------------");
        println!(" Received binary buffer {}", content);
        println!("----------------------------------------------");
        println!(" Size         : {}", buffer.len());
        println!(" Time         : {}", msg.timestamp);
        println!(" Clock        : {}", msg.clock);
        println!();
    });

    println!("Listening for binary blobs on topic 'blob'...");

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ecal::finalize();
}
