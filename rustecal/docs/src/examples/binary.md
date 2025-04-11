# Binary Message Example

## Publisher

```rust
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_bytes::BytesMessage;

fn main() {
    Ecal::initialize(Some("blob publisher"), EcalComponents::DEFAULT).unwrap();
    let pub_ = TypedPublisher::<BytesMessage>::new("blob").unwrap();
    let mut counter = 0u8;
    loop {
        let buf = vec![counter; 1024];
        pub_.send(&BytesMessage(buf));
        counter = counter.wrapping_add(1);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
```

## Subscriber

```rust
use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_bytes::BytesMessage;

fn main() {
    Ecal::initialize(Some("blob subscriber"), EcalComponents::DEFAULT).unwrap();
    let mut sub = TypedSubscriber::<BytesMessage>::new("blob").unwrap();
    sub.set_callback(|msg| {
        println!("Received blob of {} bytes", msg.msg.0.len());
    });
    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
```
