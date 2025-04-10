# rustecal – Safe Rust Bindings for Eclipse eCAL

`rustecal` is a safe and idiomatic Rust wrapper for [Eclipse eCAL](https://github.com/eclipse-ecal/ecal), designed for high-performance interprocess communication (IPC) in robotics, automotive, and embedded systems.

---

## Project Structure

| Crate                      | Description                                                 |
|---------------------------|--------------------------------------------------------------|
| `rustecal-sys`            | Low-level unsafe bindings (via `bindgen`)                    |
| `rustecal`                | Safe and idiomatic high-level wrapper for eCAL               |
| `rustecal-types-bytes`    | Typed support for raw bytes (`BytesMessage`)                 |
| `rustecal-types-string`   | Typed support for UTF-8 strings (`StringMessage`)            |
| `rustecal-types-protobuf` | Typed support for Protobuf types (`ProtobufMessage<T>`)      |
| `rustecal-samples`        | Working example binaries for string, bytes, protobuf pub/sub |

---

## Prerequisites

### Rust Toolchain

- [Rust](https://rustup.rs/) >= 1.70
- `cargo`, `rustc`

### LLVM + libclang (required for `bindgen`)

| Platform | Install                           |
|----------|-----------------------------------|
| Windows  | `choco install llvm` or use [LLVM releases](https://github.com/llvm/llvm-project/releases) |
| Linux    | `sudo apt install llvm-dev clang` |

### Environment Variable for Bindgen (Windows only)

Set `LIBCLANG_PATH` (adjust if using custom install):

```powershell
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
```

---

## eCAL Library Installation

### Windows

- Install [eCAL](https://github.com/eclipse-ecal/ecal/releases)
- Set the environment variable `ECAL_HOME`, e.g.:

```powershell
$env:ECAL_HOME = "C:\eCAL"
```

Expected structure:

```
%ECAL_HOME%/
├── include/ecal_c/      ← all C headers
└── lib/ecal_core_c.lib  ← eCAL static lib
```

### Linux

- Install system-wide from source or package
- Headers should be in:
  - `/usr/include/ecal_c/` or `/usr/local/include/ecal_c/`
- Libraries in:
  - `/usr/lib` or `/usr/local/lib` (must contain `libecal_core_c.so`)

---

## Examples

### Binary Publisher (blob_send)

```rust
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_bytes::BytesMessage;

fn main() {
    // Initialize eCAL with a custom process name and default components
    Ecal::initialize(Some("blob publisher"), EcalComponents::DEFAULT).unwrap();

    // Create a typed publisher for raw binary data on topic "blob"
    let pub_ = TypedPublisher::<BytesMessage>::new("blob").unwrap();

    let mut counter = 0u8;
    loop {
        // Create a 1024-byte buffer filled with the current counter value
        let buf = vec![counter; 1024];

        // Wrap it in a BytesMessage and send it
        pub_.send(&BytesMessage(buf));

        // Increment counter for next message
        counter = counter.wrapping_add(1);

        // Wait a bit before sending the next message
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
```

### Binary Subscriber (blob_receive)

```rust
use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_bytes::BytesMessage;

fn main() {
    // Initialize eCAL
    Ecal::initialize(Some("blob subscriber"), EcalComponents::DEFAULT).unwrap();

    // Create a typed subscriber for raw binary data
    let mut sub = TypedSubscriber::<BytesMessage>::new("blob").unwrap();

    // Set up the receive callback
    sub.set_callback(|msg| {
        // Print the size of the received buffer
        println!("Received blob of {} bytes", msg.msg.0.len());
    });

    // Keep the process alive to receive messages
    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
```

### String Publisher (hello_send)

```rust
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_string::StringMessage;

fn main() {
    // Initialize eCAL
    Ecal::initialize(Some("string publisher"), EcalComponents::DEFAULT).unwrap();

    // Create a string message publisher on topic "hello"
    let publisher = TypedPublisher::<StringMessage>::new("hello").unwrap();

    loop {
        // Format and send a message
        let msg = StringMessage(format!("Hello from Rust"));
        publisher.send(&msg);

        // Send every 500 ms
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
```

### String Subscriber (hello_receive)

```rust
use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_string::StringMessage;

fn main() {
    // Initialize eCAL
    Ecal::initialize(Some("string subscriber"), EcalComponents::DEFAULT).unwrap();

    // Create a subscriber for string messages
    let mut sub = TypedSubscriber::<StringMessage>::new("hello").unwrap();

    // Print received messages
    sub.set_callback(|msg| println!("Received: {}", msg.msg.0));

    // Keep the process alive
    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
```

### Protobuf Publisher (person_send)

```rust
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_protobuf::{ProtobufMessage, IsProtobufType};

// Generated Protobuf struct
mod person { include!(concat!(env!("OUT_DIR"), "/pb.people.rs")); }
use person::Person;

// Register the message type
impl IsProtobufType for Person {}

fn main() {
    // Initialize eCAL
    Ecal::initialize(Some("protobuf publisher"), EcalComponents::DEFAULT).unwrap();

    // Create a typed publisher for Protobuf messages
    let pub_ = TypedPublisher::<ProtobufMessage<Person>>::new("person").unwrap();

    loop {
        // Create and send a protobuf message
        let person = Person { id: 1, name: "Alice".into(), ..Default::default() };
        pub_.send(&ProtobufMessage(person));

        // Wait before sending next message
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
```

### Protobuf Subscriber (person_receive)

```rust
use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_protobuf::ProtobufMessage;

// Generated Protobuf struct
mod person { include!(concat!(env!("OUT_DIR"), "/pb.people.rs")); }
use person::Person;

fn main() {
    // Initialize eCAL
    Ecal::initialize(Some("protobuf subscriber"), EcalComponents::DEFAULT).unwrap();

    // Create a subscriber for Protobuf messages
    let mut sub = TypedSubscriber::<ProtobufMessage<Person>>::new("person").unwrap();

    // Extract and print person info from the received message
    sub.set_callback(|msg| println!("Received person: {}", msg.msg.0.name));

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
```

---

## Supported Message Types

- `BytesMessage` – Arbitrary binary data
- `StringMessage` – UTF-8 encoded text
- `ProtobufMessage<T>` – Protobuf-encoded structs (via prost)

Each type is provided by an external crate:  
- `rustecal-types-bytes`  
- `rustecal-types-string`  
- `rustecal-types-protobuf`

---

## Workspace Layout

```
your_workspace/
├── rustecal-sys/                  # Raw C FFI bindings
├── rustecal/                      # High-level safe API
├── rustecal-types-bytes/          # BytesMessage wrapper crate
├── rustecal-types-string/         # StringMessage wrapper crate
├── rustecal-types-protobuf/       # ProtobufMessage wrapper crate
└── rustecal-samples/              # Sample applications
    └── pubsub/
        ├── blob_send/             # Sends byte messages
        ├── blob_receive/          # Receives byte messages
        ├── hello_send/            # Sends string messages
        ├── hello_receive/         # Receives string messages
        ├── person_send/           # Sends protobuf messages
        └── person_receive/        # Receives protobuf messages
```

---

## Roadmap

- [x] Cross-platform build support (Windows + Linux)
- [x] Safe initialization/finalization
- [x] Publisher / Subscriber APIs
- [x] Typed pub/sub with generic `T: Message`
- [x] `StringMessage`, `BytesMessage`, `ProtobufMessage<T>` types
- [x] External crates per type support to avoid core dependency bloat
- [x] Closure-based callbacks
- [ ] Protobuf descriptor support
- [ ] eCAL Service support (client/server)
- [ ] Monitoring/logging API
- [ ] Configuration tools

---

## Author

Created by Rex Schilasky  
🚗 Automotive | 🧠 SDV | 🛠️ Rust | 🚀 IPC

---

## License

Licensed under the [Apache 2.0 License](LICENSE)
