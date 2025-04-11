# Project Structure

| Crate | Description |
|-------|-------------|
| `rustecal-sys` | Low-level unsafe bindings (via `bindgen`) |
| `rustecal` | Safe and idiomatic high-level wrapper |
| `rustecal-types-bytes` | Support for raw byte messages |
| `rustecal-types-string` | UTF-8 string message support |
| `rustecal-types-protobuf` | Protobuf support using `prost` |
| `rustecal-samples` | Working binary examples |

## Workspace Layout

```
your_workspace/
├── rustecal/
├── rustecal-sys/
├── rustecal-types-bytes/
├── rustecal-types-string/
├── rustecal-types-protobuf/
└── rustecal-samples/
    └── pubsub/
        ├── blob_send/
        ├── blob_receive/
        ├── hello_send/
        ├── hello_receive/
        ├── person_send/
        └── person_receive/
```
