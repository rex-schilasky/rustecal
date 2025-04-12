# Supported Message Types

`rustecal` supports message types through wrapper structs:

## `StringMessage`

Used for UTF-8 string topics.

## `BytesMessage`

Used for binary `Vec<u8>` payloads.

## `ProtobufMessage<T>`

Supports publishing/receiving of Protobuf types that implement `Message` and `Default`.

```rust
use rustecal::types::ProtobufMessage;
use my_proto::MyProto;

let pub = Publisher::<ProtobufMessage<MyProto>>::builder("proto_topic").create()?;
```
