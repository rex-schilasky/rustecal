# rustecal

[![Docs](https://img.shields.io/badge/mdBook-docs-blue)](https://rex-schilasky.github.io/rustecal/)

> A safe and idiomatic Rust wrapper for the [Eclipse eCAL](https://github.com/eclipse-ecal/ecal) high-performance middleware.  
> Offers modular support for `String`, `Bytes`, and `Protobuf` types and integrates cleanly across platforms.

---

### 📚 Documentation

📖 **View the full documentation here**:  
👉 [https://rex-schilasky.github.io/rustecal](https://rex-schilasky.github.io/rustecal)

---

### 🧩 Project Structure

- `rustecal` – Core wrapper and safe pub/sub API
- `rustecal-types-string` – `String` message support
- `rustecal-types-bytes` – `Vec<u8>` message support
- `rustecal-types-protobuf` – `Protobuf` message support via `prost`
- `rustecal-sys` – Low-level FFI bindings generated with `bindgen`

---

### 🚀 Getting Started

Coming soon in full detail in the docs.  
For now, try the examples in the `examples/` directory:

```bash
cargo run --example hello_send
cargo run --example hello_receive
