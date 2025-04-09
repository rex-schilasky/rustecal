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
use rustecal::pubsub::typed_subscriber::IsProtobufType;

use people::Person;
impl IsProtobufType for people::Person {}

fn main() {
    Ecal::initialize(Some("person receive rust"), EcalComponents::DEFAULT).unwrap();

    let mut subscriber = TypedSubscriber::<Person>::new("person").unwrap();

    subscriber.set_callback(|msg: Person| {
        println!("person id    : {}", msg.id);
        println!("person name  : {}", msg.name);
        println!("person stype : {}", msg.stype);
        println!("person email : {}", msg.email);
        println!("dog.name     : {}", msg.dog.as_ref().map_or("", |d| &d.name));
        println!("house.rooms  : {}", msg.house.as_ref().map_or(0, |h| h.rooms));
        println!();
    });

    println!("Waiting for person messages...");

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}