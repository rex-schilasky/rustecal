# Message Types

rustecal supports several message formats, wrapped in type-safe crates:

## `BytesMessage`

Raw byte buffers.

```rust
use rustecal_types_bytes::BytesMessage;

let msg = BytesMessage(vec![1, 2, 3]);
```

## `StringMessage`

UTF-8 encoded Rust strings.

```rust
use rustecal_types_string::StringMessage;

let msg = StringMessage("hello world".into());
```

## `ProtobufMessage<T>`

Support for `prost::Message`-based Protobuf types.

```rust
use rustecal_types_protobuf::ProtobufMessage;
use myproto::MyMessage;

let msg = ProtobufMessage(MyMessage::default());
```