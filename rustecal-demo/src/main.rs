use rustecal::{Ecal, Publisher};
use rustecal::pubsub::types::DataTypeInfo;
use std::{thread, time::Duration};

fn main() {
    Ecal::initialize(Some("minimal string publisher rust")).expect("eCAL initialization failed");

    let datatype = DataTypeInfo {
        encoding:   "utf-8".to_string(),
        type_name:  "string".to_string(),
        descriptor: b"".to_vec(),
    };

    let publisher = Publisher::new("Hello", datatype).expect("Failed to create publisher");

    let mut i = 0;
    while Ecal::ok() {
        i += 1;
        let msg = format!("HELLO WORLD FROM RUST ({})", i);
        publisher.send(msg.as_bytes());
        println!("Sent: {}", msg);
        thread::sleep(Duration::from_millis(500));
    }

    Ecal::finalize();
}
