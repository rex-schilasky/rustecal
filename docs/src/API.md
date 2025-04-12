# API Documentation

This section provides reference documentation for the core API of **rustecal**.

## Ecal Core

- **Ecal::initialize(unit_name, components)**  
  Initializes the eCAL API with a given optional unit name and a bitflag of components (see [EcalComponents](./Components.md)).  
  - **Arguments:**  
    - `unit_name`: `Option<&str>` – An optional string that identifies the eCAL unit.  
    - `components`: `EcalComponents` – Bitflag flags to specify which components to initialize (default: publisher, subscriber, service, logging, and timesync).  
  - **Returns:** `Result<(), i32>`  
    Returns `Ok(())` on success or an error code if initialization fails.

- **Ecal::finalize()**  
  Finalizes the eCAL API and releases all associated resources.

- **Ecal::ok() -> bool**  
  Checks if eCAL is in a valid state.

## Publisher API

See [Publisher Documentation](Publisher.md) for details on how to create a publisher, send messages (with and without timestamps), and query topic metadata.

## Subscriber API

See [Subscriber Documentation](Subscriber.md) for details on setting up a subscriber, receiving messages, and accessing topic metadata.

## Typed Pub/Sub

The typed publisher and subscriber API wrap the low-level functions and add type safety. Implement the traits:
- [`PublisherMessage`](#publishermessage)
- [`SubscriberMessage`](#subscribermessage)

to support your message types. There are helper crates provided for common types:
- **rustecal-types-string** – for UTF‑8 strings.
- **rustecal-types-bytes** – for raw binary messages.
- **rustecal-types-protobuf** – for Protobuf‑encoded messages.

*See the sample applications in the `rustecal-samples/pubsub` directory for practical examples.*
