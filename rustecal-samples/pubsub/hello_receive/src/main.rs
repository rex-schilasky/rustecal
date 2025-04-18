use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal::pubsub::typed_subscriber::Received;
use rustecal_types_string::StringMessage;

fn main() {
    Ecal::initialize(Some("hello receive rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let mut subscriber = TypedSubscriber::<StringMessage>::new("hello")
        .expect("Failed to create subscriber");

    subscriber.set_callback(|msg: Received<StringMessage>| {
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
        println!("message      : {}", msg.msg.0);
        println!("------------------------------------------\n");
    });

    println!("Waiting for messages on topic 'hello'...");

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ecal::finalize();
}
