mod people {
    include!(concat!(env!("OUT_DIR"), "/pb.people.rs"));
}
mod animal {
    include!(concat!(env!("OUT_DIR"), "/pb.animal.rs"));
}
mod environment {
    include!(concat!(env!("OUT_DIR"), "/pb.environment.rs"));
}

use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_protobuf::{ProtobufMessage, IsProtobufType};

use people::Person;

// Implement marker trait to enable ProtobufMessage<Person>
impl IsProtobufType for Person {}

fn main() {
    Ecal::initialize(Some("person protobuf subscriber rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let mut subscriber = TypedSubscriber::<ProtobufMessage<Person>>::new("person")
        .expect("Failed to create subscriber");

    subscriber.set_callback(|msg: ProtobufMessage<Person>| {
        let person = msg.0;
        println!("person id    : {}", person.id);
        println!("person name  : {}", person.name);
        println!("person stype : {}", person.stype);
        println!("person email : {}", person.email);
        println!("dog.name     : {}", person.dog.as_ref().map_or("", |d| &d.name));
        println!("house.rooms  : {}", person.house.as_ref().map_or(0, |h| h.rooms));
        println!();
    });

    println!("Waiting for person messages...");

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}
