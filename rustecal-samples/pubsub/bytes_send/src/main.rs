use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_bytes::BytesMessage;

fn main() {
    Ecal::initialize(Some("bytes publisher rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let publisher = TypedPublisher::<BytesMessage>::new("bytes")
        .expect("Failed to create publisher");

    let mut cnt = 0;
    while Ecal::ok() {
        cnt += 1;
        let payload = format!("Bytes Packet #{}", cnt).into_bytes();
        publisher.send(&BytesMessage(payload.clone()));
        println!("Sent: {:?}", String::from_utf8_lossy(&payload));
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}
