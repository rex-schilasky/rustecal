# rustecal – Safe Rust Bindings for Eclipse eCAL 🚀

`rustecal` is a safe and idiomatic Rust wrapper for the [Eclipse eCAL](https://github.com/eclipse-ecal/ecal) C API, designed for high-performance interprocess communication (IPC) in robotics, automotive, and embedded systems.

This project consists of three Rust crates:

```
📦 rustecal-sys   – raw FFI bindings to the eCAL C API (generated via bindgen)
📦 rustecal       – safe high-level Rust wrapper over rustecal-sys
📦 rustecal-demo  – example app using rustecal
```

---

## 📦 Project Structure

| Crate             | Description                                      |
|------------------|--------------------------------------------------|
| `rustecal-sys`    | Low-level unsafe bindings (via `bindgen`)        |
| `rustecal`        | Safe Rust abstraction for eCAL users             |
| `rustecal-demo`   | Example application using `rustecal::Ecal`       |

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

### 🔷 Windows

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

### 🔷 Linux

- Install system-wide from source or use package if available
- Headers must be in:
  - `/usr/include/ecal_c/` or `/usr/local/include/ecal_c/`
- Libraries in:
  - `/usr/lib` or `/usr/local/lib` containing `libecal_core_c.so`

---

## 🔨 Build Instructions

### 🔷 On Windows

```powershell
cd rustecal-sys
cargo build

cd ../rustecal
cargo build

cd ../rustecal-demo
cargo run
```

### 🔷 On Linux

```bash
cd rustecal-sys
cargo build

cd ../rustecal
cargo build

cd ../rustecal-demo
cargo run
```

---

## 🚀 Example Usage

Inside `rustecal-demo/src/main.rs`:

```rust
fn main() {
    rustecal::Ecal::initialize(Some("rustecal_node"))
        .expect("Failed to init eCAL");

    println!("✅ eCAL is running via Rust");

    rustecal::Ecal::finalize();
}
```

---

## 📁 Workspace Layout

```
your_workspace/
├── rustecal-sys/     # Raw bindings via bindgen
├── rustecal/         # Safe Rust wrapper API
└── rustecal-demo/    # Sample usage app
```

Optional: set up a top-level workspace `Cargo.toml` if desired.

---

## 🧱 Roadmap

- [x] Cross-platform build support (Windows + Linux)
- [x] Safe initialization/finalization
- [ ] Publisher / Subscriber API
- [ ] Service client/server support
- [ ] Configuration handling
- [ ] Monitoring / logging utilities
- [ ] Protobuf support via `prost` or `nanopb`

---

## 👨‍💻 Author

Created by Rex Schilasky  
🚗 Automotive | 🧠 SDV | 🧰 Rust | 🛰️ IPC

---

## 📄 License

Licensed under the [Apache 2.0 License](LICENSE).
