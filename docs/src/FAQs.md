# FAQs

### Q: Why does rustecal separate message types into external crates?
A: This design helps keep the core library lightweight and minimizes dependencies. Each message type (e.g., string, bytes, protobuf) is supported by its own crate.

### Q: How do I add support for another message type?
A: Implement the [`PublisherMessage`] and [`SubscriberMessage`] traits for your custom type, then create an external crate if desired.

### Q: Where can I find sample applications?
A: Refer to the `rustecal-samples/pubsub` directory for multiple examples demonstrating string, bytes, and Protobuf-based pub/sub.
