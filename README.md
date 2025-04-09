# rustecal – Safe Rust Bindings for Eclipse eCAL 🚀

`rustecal` is a safe and idiomatic Rust wrapper for the [Eclipse eCAL](https://github.com/eclipse-ecal/ecal) C API, designed for high-performance interprocess communication (IPC) in robotics, automotive, and embedded systems.

This project consists of three Rust crates:

```
📦 rustecal-sys       – raw FFI bindings to the eCAL C API (generated via bindgen)
📦 rustecal           – safe high-level Rust wrapper over rustecal-sys
📦 rustecal-samples   – sample applications using rustecal (pub/sub, services, etc.)
```

---

## 📦 Project Structure

| Crate               | Description                                         |
|--------------------|-----------------------------------------------------|
| `rustecal-sys`     | Low-level unsafe bindings (via `bindgen`)           |
| `rustecal`         | Safe Rust abstraction for eCAL users                |
| `rustecal-samples` | Sample apps demonstrating pub/sub and other features|

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

## 📈 Build Instructions

### 🔹 On Windows

```powershell
cd rustecal-sys
cargo build

cd ../rustecal
cargo build

cd ../rustecal-samples/pubsub/hello_send
cargo run

cd ../hello_receive
cargo run
```

### 🔹 On Linux

```bash
cd rustecal-sys
cargo build

cd ../rustecal
cargo build

cd ../rustecal-samples/pubsub/hello_send
cargo run

cd ../hello_receive
cargo run
```

---

## 🚀 Example: Typed String Publisher

```rust
use rustecal::{Ecal, TypedPublisher, EcalComponents};

fn main() {
    Ecal::initialize(Some("minimal string publisher"), EcalComponents::DEFAULT).unwrap();

    let publisher = TypedPublisher::<String>::new("hello").unwrap();

    let mut cnt = 0;
    while Ecal::ok() {
        let msg = format!("HELLO WORLD FROM RUST ({})", cnt);
        publisher.send(&msg);
        println!("Sent: {}", msg);
        cnt += 1;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}
```

---

## 🚀 Example: Typed String Subscriber

```rust
use rustecal::{Ecal, TypedSubscriber, EcalComponents};

fn main() {
    Ecal::initialize(Some("minimal string subscriber"), EcalComponents::DEFAULT).unwrap();

    let mut subscriber = TypedSubscriber::<String>::new("hello")
        .expect("Failed to create subscriber");

    subscriber.set_callback(|msg| {
        println!("Received: {}", msg);
    });

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}
```

---

## ✅ Supported Message Types

- `String` – UTF-8 encoded text (encoding: `"utf-8"`)
- `Vec<u8>` – Raw binary data (encoding: `"raw"`)
- `prost::Message` (Protobuf) – Requires `IsProtobufType` marker trait

You can add your own types by implementing the `PublisherMessage` / `SubscriberMessage` traits.

---

## 📁 Workspace Layout

```
your_workspace/
├── rustecal-sys/                # Raw bindings via bindgen
├── rustecal/                    # Safe Rust wrapper API
└── rustecal-samples/            # Sample applications
    └── pubsub/
        ├── hello_send/          # Sends hello world messages
        └── hello_receive/       # Receives hello world messages
```

---

## 🧱 Roadmap

- [x] Cross-platform build support (Windows + Linux)
- [x] Safe initialization/finalization
- [x] Publisher / Subscriber APIs
- [x] Generic typed pub/sub (`String`, `Vec<u8>`, `prost::Message`)
- [x] Closure-based callback support
- [ ] Service client/server support
- [ ] Configuration module
- [ ] Monitoring / logging utilities
- [ ] Protobuf descriptor support
- [ ] Performance benchmarking tools

---

## 👨‍💻 Author

Created by Rex Schilasky  
🚗 Automotive | 🧠 SDV | 🛠️ Rust | 🚀 IPC

---

## 📄 License

Licensed under the [Apache 2.0 License](LICENSE).
