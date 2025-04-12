# Typed Publisher

The `Publisher<T>` allows you to publish messages of type `T` on a topic.

## Example

```rust
use rustecal::pubsub::Publisher;
use rustecal::types::StringMessage;

let pub = Publisher::<StringMessage>::builder("my_topic").create()?;
pub.send("Rust rocks!")?;
```
