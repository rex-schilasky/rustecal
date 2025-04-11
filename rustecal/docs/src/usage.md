# Usage Guide

## Initialization

Before using any publisher or subscriber, initialize the eCAL runtime:

```rust
use rustecal::{Ecal, EcalComponents};

fn main() {
    Ecal::initialize(Some("my_node"), EcalComponents::DEFAULT).unwrap();
    // ...
    Ecal::finalize();
}
```

## Publishing Strings

```rust
use rustecal::{TypedPublisher};
use rustecal_types_string::StringMessage;

let pub_ = TypedPublisher::<StringMessage>::new("hello").unwrap();
pub_.send(&StringMessage("Hello from Rust".into()));
```

## Subscribing to Strings

```rust
use rustecal::{TypedSubscriber};
use rustecal_types_string::StringMessage;

let mut sub = TypedSubscriber::<StringMessage>::new("hello").unwrap();
sub.set_callback(|msg| {
    println!("Received: {}", msg.msg.0);
});
```