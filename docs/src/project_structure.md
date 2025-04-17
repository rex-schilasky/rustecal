# Project Structure

This workspace is organized into several purpose-specific crates to provide a modular, maintainable API for eCAL:

| Crate                     | Description                                              |
|---------------------------|----------------------------------------------------------|
| `rustecal`                | **Meta‑crate**: re‑exports core, pub/sub, and service APIs via feature flags (`pubsub`, `service`) |
| `rustecal-core`           | eCAL initialization, shutdown, and shared common types    |
| `rustecal-pubsub`         | Typed and untyped Publisher/Subscriber API               |
| `rustecal-service`        | RPC Service server & client API                          |
| `rustecal-sys`            | Low‑level FFI bindings to the eCAL C API                 |
| `rustecal-types-string`   | Helper: UTF-8 string message wrapper for typed pub/sub   |
| `rustecal-types-bytes`    | Helper: raw byte vector message wrapper                  |
| `rustecal-types-protobuf` | Helper: Protobuf message wrapper (using `prost`)         |
| `rustecal-samples`        | Example binaries demonstrating pub/sub and RPC usage      |

## Workspace Layout

```text
your_workspace/
├── Cargo.toml              # workspace manifest
├── rustecal/               # meta‑crate (feature‑gated)
├── rustecal-core/          # core init + types
├── rustecal-pubsub/        # pub/sub API
├── rustecal-service/       # service RPC API
├── rustecal-sys/           # raw C bindings
├── rustecal-types-string/
├── rustecal-types-bytes/
├── rustecal-types-protobuf/
└── rustecal-samples/       # examples
    ├── pubsub/
    │   ├── hello_send/
    │   ├── hello_receive/
    │   ├── blob_send/
    │   ├── blob_receive/
    │   ├── person_send/
    │   └── person_receive/
    └── service/
        ├── mirror_server/
        ├── mirror_client/
        └── mirror_client_instances/
