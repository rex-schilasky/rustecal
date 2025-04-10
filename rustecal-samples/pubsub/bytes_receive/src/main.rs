use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_bytes::BytesMessage;

fn main() {
    Ecal::initialize(Some("bytes subscriber rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let mut subscriber = TypedSubscriber::<BytesMessage>::new("blob")
        .expect("Failed to create subscriber");

    subscriber.set_callback(|msg: BytesMessage| {
        let buffer = msg.0;
        if buffer.is_empty() {
            return;
        }

        let content = buffer[0] as u8;

        println!("----------------------------------------------");
        println!(" Received binary buffer {}", content);
        println!("----------------------------------------------");
        println!(" Size         : {}", buffer.len());
        // NOTE: eCAL C API does not currently expose timestamp or clock in the typed API
        // println!(" Time         : N/A");
        // println!(" Clock        : N/A\n");
    });

    println!("Listening for binary blobs on topic 'blob'...");

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}
