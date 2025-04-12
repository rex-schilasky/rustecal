# Typed Subscriber

The `Subscriber<T>` enables you to subscribe to messages of type `T` on a topic.

## Example

```rust
use rustecal::pubsub::Subscriber;
use rustecal::types::StringMessage;

let sub = Subscriber::<StringMessage>::builder("my_topic").create()?;
sub.set_callback(|msg| {
    println!("Received: {}", msg.data());
});
```
