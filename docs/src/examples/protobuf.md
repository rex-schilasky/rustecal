# Protobuf Message Example

## Publisher

```rust
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_protobuf::{ProtobufMessage, IsProtobufType};
mod person { include!(concat!(env!("OUT_DIR"), "/pb.people.rs")); }
use person::Person;

impl IsProtobufType for Person {}

fn main() {
    Ecal::initialize(Some("protobuf publisher"), EcalComponents::DEFAULT).unwrap();
    let pub_ = TypedPublisher::<ProtobufMessage<Person>>::new("person").unwrap();
    loop {
        let person = Person { id: 1, name: "Alice".into(), ..Default::default() };
        pub_.send(&ProtobufMessage(person));
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
```

## Subscriber

```rust
use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_protobuf::ProtobufMessage;
mod person { include!(concat!(env!("OUT_DIR"), "/pb.people.rs")); }
use person::Person;

fn main() {
    Ecal::initialize(Some("protobuf subscriber"), EcalComponents::DEFAULT).unwrap();
    let mut sub = TypedSubscriber::<ProtobufMessage<Person>>::new("person").unwrap();
    sub.set_callback(|msg| println!("Received person: {}", msg.msg.0.name));
    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
```
