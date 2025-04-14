# rustecal

[![Build Status](https://github.com/rex-schilasky/rustecal/actions/workflows/ci.yml/badge.svg)](https://github.com/rex-schilasky/rustecal/actions)
[![Docs](https://img.shields.io/badge/docs-mdbook-blue)](https://rex-schilasky.github.io/rustecal/)

Safe and idiomatic Rust bindings for the [eCAL](https://github.com/eclipse-ecal/ecal) middleware — a high-performance IPC framework designed for distributed real-time systems.

---

## Features

- 📡 High-performance publish/subscribe middleware (based on eCAL)
- 🦀 Idiomatic Rust API over eCAL C-API
- 💬 Type-safe messaging for:
  - `StringMessage`
  - `BytesMessage`
  - `ProtobufMessage<T>` (based on `prost`)
- 🧪 Works on Linux and Windows (via `bindgen` + `cc`)
- 📖 Modular message support via `rustecal-types-*` crates

---

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rustecal = { path = "path/to/rustecal" }
rustecal-types-string = { path = "path/to/rustecal-types-string" }
```

Example (sending a string message):

```rust
use rustecal::pubsub::Publisher;
use rustecal::types::StringMessage;

let _ecal = rustecal::Ecal::initialize("hello_send")?;

let publisher = Publisher::<StringMessage>::builder("hello_topic").create()?;
publisher.send("Hello from Rust!")?;
```

Example (receiving a message):

```rust
use rustecal::pubsub::Subscriber;
use rustecal::types::StringMessage;

let _ecal = rustecal::Ecal::initialize("hello_receive")?;

let subscriber = Subscriber::<StringMessage>::builder("hello_topic").create()?;
subscriber.set_callback(|msg| {
    println!("Received: {}", msg.data());
});
```

---

## Crate Structure

- `rustecal`: core eCAL bindings and idiomatic API
- `rustecal-sys`: low-level `bindgen` generated FFI bindings
- `rustecal-types-string`, `rustecal-types-bytes`, `rustecal-types-protobuf`: message wrapper crates

---

## Documentation

📚 Full user guide: [https://rex-schilasky.github.io/rustecal](https://rex-schilasky.github.io/rustecal)

```bash
cd docs/
mdbook serve
```

---

## License

Licensed under Apache-2.0 or MIT.

---

## Maintainer

[Rex Schilasky](https://github.com/rex-schilasky)
