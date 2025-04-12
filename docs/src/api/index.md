# API Documentation

Welcome to the `rustecal` API documentation. This section provides an overview of the main types and traits used to interact with the eCAL communication system through safe and idiomatic Rust APIs.

## Modules Overview

- [`Ecal`](../api/ecal.md) — Lifecycle manager for eCAL initialization and finalization.
- [`Publisher<T>`](./publisher.md) — Generic typed publisher used to send messages on a topic.
- [`Subscriber<T>`](./subscriber.md) — Generic typed subscriber used to receive messages.
- [`MessageType`](./message_types.md) — Trait for enabling custom serialization of types.
- Message Wrappers:
  - [`StringMessage`](./message_types.md#stringmessage)
  - [`BytesMessage`](./message_types.md#bytesmessage)
  - [`ProtobufMessage<T>`](./message_types.md#protobufmessaget)

Each module has its own documentation page with examples.

## Usage Highlights

```rust
let _ecal = Ecal::initialize("my_app")?;
let pub = Publisher::<StringMessage>::builder("my_topic").create()?;
pub.send("Hello from Rust!")?;
```

---

Explore the individual components in detail:
- [Ecal Lifecycle](./ecal.md)
- [Typed Publisher](./publisher.md)
- [Typed Subscriber](./subscriber.md)
- [Supported Message Types](./message_types.md)
