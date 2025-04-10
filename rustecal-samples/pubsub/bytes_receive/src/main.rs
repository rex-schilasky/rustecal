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

        println!("------------------------------------------");
        println!(" HEAD ");
        println!("------------------------------------------");
        println!("topic name   : {}", msg.topic_name);
        println!("encoding     : {}", msg.encoding);
        println!("type name    : {}", msg.type_name);
        println!("topic time   : {}", msg.timestamp);
        println!("topic clock  : {}", msg.clock);
        println!("------------------------------------------");
        println!(" CONTENT ");
        println!("------------------------------------------");
        println!("binary value : {}", content);
        println!("buffer size  : {}", buffer.len());
        println!("------------------------------------------\n");
    });

    println!("Waiting for binary blobs on topic 'blob'...");

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ecal::finalize();
}
