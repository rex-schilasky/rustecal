# String Message Example

## Publisher

```rust
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_string::StringMessage;

fn main() {
    Ecal::initialize(Some("string publisher"), EcalComponents::DEFAULT).unwrap();
    let publisher = TypedPublisher::<StringMessage>::new("hello").unwrap();
    while Ecal::ok() {
        let msg = StringMessage(format!("Hello from Rust"));
        publisher.send(&msg);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
```

## Subscriber

```rust
use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_string::StringMessage;

fn main() {
    Ecal::initialize(Some("string subscriber"), EcalComponents::DEFAULT).unwrap();
    let mut sub = TypedSubscriber::<StringMessage>::new("hello").unwrap();
    sub.set_callback(|msg| println!("Received: {}", msg.msg.0));
    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
```
