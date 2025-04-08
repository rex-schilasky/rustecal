use rustecal::{Ecal, Publisher};
use rustecal::pubsub::types::DataTypeInfo;
use std::{thread, time::Duration};

fn main() {
    Ecal::initialize(Some("rustecal_demo")).expect("eCAL initialization failed");

    let datatype = DataTypeInfo {
        encoding: "utf-8".to_string(),
        type_name: "string".to_string(),
        descriptor: b"".to_vec(),
    };

    let publisher = Publisher::new("demo_topic", datatype).expect("Failed to create publisher");

    for i in 0..100 {
        let msg = format!("Hello from rustecal! #{}", i);
        publisher.send(msg.as_bytes()).unwrap();
        println!("📤 Sent: {}", msg);
        thread::sleep(Duration::from_millis(500));
    }

    Ecal::finalize();
}
