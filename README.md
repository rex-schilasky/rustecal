# rustecal – Safe Rust Bindings for Eclipse eCAL 🚀

`rustecal` is a safe and idiomatic Rust wrapper for the [Eclipse eCAL](https://github.com/eclipse-ecal/ecal) C API, designed for high-performance interprocess communication (IPC) in robotics, automotive, and embedded systems.

This project consists of multiple Rust crates:

```
📦 rustecal-sys             – raw FFI bindings to the eCAL C API (generated via bindgen)
📦 rustecal                 – safe high-level Rust wrapper over rustecal-sys
📦 rustecal-types-string    – String message support for rustecal pub/sub
📦 rustecal-types-bytes     – Bytes (Vec<u8>) message support for rustecal pub/sub
📦 rustecal-types-protobuf  – Protobuf message support (via prost)
📦 rustecal-samples         – Sample applications using rustecal (pub/sub)
```

---

## 📦 Project Structure

| Crate                      | Description                                               |
|---------------------------|-----------------------------------------------------------|
| `rustecal-sys`            | Low-level unsafe bindings (via `bindgen`)                 |
| `rustecal`                | Safe and idiomatic high-level wrapper for eCAL            |
| `rustecal-types-string`   | Typed support for UTF-8 strings (`StringMessage`)         |
| `rustecal-types-bytes`    | Typed support for raw bytes (`BytesMessage`)              |
| `rustecal-types-protobuf` | Typed support for Protobuf types (`ProtobufMessage<T>`)   |
| `rustecal-samples`        | Working example binaries for string, bytes, protobuf pub/sub |

---

## 🛠️ Prerequisites

### ✅ Rust Toolchain

- [Rust](https://rustup.rs/) >= 1.70
- `cargo`, `rustc`

### ✅ LLVM + libclang (required for `bindgen`)

| Platform | Install                        |
|----------|--------------------------------|
| Windows  | `choco install llvm` or use [LLVM releases](https://github.com/llvm/llvm-project/releases) |
| Linux    | `sudo apt install llvm-dev clang` |

### ✅ Environment Variable for Bindgen (Windows only)

Set `LIBCLANG_PATH` (adjust if using custom install):

```powershell
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
```

---

## 📦 eCAL Library Installation

### 🔹 Windows

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

### 🔹 Linux

- Install system-wide from source or package
- Headers should be in:
  - `/usr/include/ecal_c/` or `/usr/local/include/ecal_c/`
- Libraries in:
  - `/usr/lib` or `/usr/local/lib` (must contain `libecal_core_c.so`)

---

## 🧪 Examples

### ✅ StringMessage Publisher

```rust
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_string::StringMessage;

fn main() {
    Ecal::initialize(Some("hello string publisher rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let publisher = TypedPublisher::<StringMessage>::new("hello")
        .expect("Failed to create publisher");

    let mut cnt = 0;
    while Ecal::ok() {
        let msg = format!("HELLO WORLD FROM RUST ({})", cnt);
        publisher.send(&StringMessage(msg));
        cnt += 1;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}
```

### ✅ StringMessage Subscriber

```rust
use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_string::StringMessage;

fn main() {
    Ecal::initialize(Some("hello string subscriber rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let mut subscriber = TypedSubscriber::<StringMessage>::new("hello")
        .expect("Failed to create subscriber");

    subscriber.set_callback(|msg: StringMessage| {
        println!("Received: {}", msg.0);
    });

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}
```

---

### ✅ BytesMessage Publisher

```rust
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_bytes::BytesMessage;

fn main() {
    Ecal::initialize(Some("bytes publisher rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let publisher = TypedPublisher::<BytesMessage>::new("data")
        .expect("Failed to create publisher");

    let data = BytesMessage(vec![0xde, 0xad, 0xbe, 0xef]);
    publisher.send(&data);

    Ecal::finalize();
}
```

### ✅ BytesMessage Subscriber

```rust
use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_bytes::BytesMessage;

fn main() {
    Ecal::initialize(Some("bytes subscriber rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let mut subscriber = TypedSubscriber::<BytesMessage>::new("data")
        .expect("Failed to create subscriber");

    subscriber.set_callback(|msg: BytesMessage| {
        println!("Received bytes: {:x?}", msg.0);
    });

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}
```

---

## ✅ Supported Message Types

- `StringMessage` – UTF-8 encoded text
- `BytesMessage` – Arbitrary binary data
- `ProtobufMessage<T>` – Protobuf-encoded structs (via prost)

Each type is provided by an external crate:  
- `rustecal-types-string`  
- `rustecal-types-bytes`  
- `rustecal-types-protobuf`

---

## 📁 Workspace Layout

```
your_workspace/
├── rustecal-sys/                  # Raw C FFI bindings
├── rustecal/                      # High-level safe API
├── rustecal-types-string/         # StringMessage wrapper crate
├── rustecal-types-bytes/          # BytesMessage wrapper crate
├── rustecal-types-protobuf/       # ProtobufMessage wrapper crate
└── rustecal-samples/              # Sample applications
    └── pubsub/
        ├── hello_send/            # Sends string messages
        ├── hello_receive/         # Receives string messages
        ├── bytes_send/            # Sends byte messages
        ├── bytes_receive/         # Receives byte messages
        ├── person_send/           # Sends protobuf messages
        └── person_receive/        # Receives protobuf messages
```

---

## 🧱 Roadmap

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

## 👨‍💻 Author

Created by Rex Schilasky  
🚗 Automotive | 🧠 SDV | 🛠️ Rust | 🚀 IPC

---

## 📄 License

Licensed under the [Apache 2.0 License](LICENSE)
