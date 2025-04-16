# rustecal

[![Docs](https://img.shields.io/badge/docs-mdbook-blue)](https://rex-schilasky.github.io/rustecal/)

Safe and idiomatic Rust bindings for the [eCAL](https://github.com/eclipse-ecal/ecal) middleware — a high-performance IPC framework designed for distributed real-time systems.

---

## Features

- Idiomatic Rust interface to the eCAL C API
- Zero-copy shared memory transport
- Type-safe publish/subscribe and service communication
- Modular type support: String, Binary, Protobuf
- Fully runtime-compatible with C++ eCAL systems

---

## Examples

### Publisher

```rust
use rustecal::{Ecal, EcalComponents};
use rustecal::pubsub::Publisher;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("rust publisher"), EcalComponents::DEFAULT)?;
    let mut pub = Publisher::<String>::new("chatter")?;

    loop {
        pub.send("Hello from Rust!")?;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
```

---

### Subscriber

```rust
use rustecal::{Ecal, EcalComponents};
use rustecal::pubsub::Subscriber;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("rust subscriber"), EcalComponents::DEFAULT)?;
    let sub = Subscriber::<String>::new("chatter")?;

    sub.set_callback(|msg| {
        println!("Received: {}", msg.payload);
    })?;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
```

---

### Service Server

```rust
use rustecal::{Ecal, EcalComponents};
use rustecal::service::server::ServiceServer;
use rustecal::service::types::MethodInfo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("mirror server"), EcalComponents::DEFAULT)?;
    let mut server = ServiceServer::new("mirror")?;

    server.add_method("reverse", Box::new(|_info: MethodInfo, req: &[u8]| {
        let mut reversed = req.to_vec();
        reversed.reverse();
        reversed
    }))?;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
```

---

### Service Client

```rust
use rustecal::{Ecal, EcalComponents};
use rustecal::service::client::ServiceClient;
use rustecal::service::types::ServiceRequest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("mirror client"), EcalComponents::DEFAULT)?;
    let client = ServiceClient::new("mirror")?;

    let request = ServiceRequest {
        payload: b"stressed".to_vec(),
    };

    if let Some(response) = client.call("reverse", request, Some(1000)) {
        println!("Reversed: {}", String::from_utf8_lossy(&response.payload));
    } else {
        println!("No response received.");
    }

    Ok(())
}
```

---

## Documentation

- 📘 API Docs: [docs.rs/rustecal](https://docs.rs/rustecal)
- 📖 Guide & Examples: see `docs/` (mdBook)

---

## License

Licensed under the Apache License 2.0 (see [LICENSE](./LICENSE))  
© 2024–2025 Eclipse Contributors / Rex Schilasky
