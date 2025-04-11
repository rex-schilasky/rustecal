use crate::pubsub::publisher::Publisher;
use crate::pubsub::types::DataTypeInfo;
use std::marker::PhantomData;

/// Trait for types that can be published via `TypedPublisher`.
///
/// This must be implemented by all message types that are intended to be sent
/// through the typed publisher API.
///
/// Implementors must provide:
/// - A static method `datatype()` returning the associated encoding, type name,
///   and optional descriptor.
/// - A method `to_bytes()` that serializes the message into a binary buffer.
pub trait PublisherMessage {
    fn datatype() -> DataTypeInfo;
    fn to_bytes(&self) -> Vec<u8>;
}

/// Type-safe, high-level wrapper around an `eCAL` publisher.
///
/// This struct encapsulates a raw `Publisher` and associates it with a specific
/// message type `T`. It ensures that only compatible messages are published,
/// and automatically handles serialization and topic metadata.
pub struct TypedPublisher<T: PublisherMessage> {
    publisher: Publisher,
    _phantom: PhantomData<T>,
}

impl<T: PublisherMessage> TypedPublisher<T> {
    /// Creates a new typed publisher for the specified topic.
    ///
    /// # Arguments
    ///
    /// * `topic_name` - The name of the topic to publish to.
    ///
    /// # Returns
    ///
    /// A `TypedPublisher<T>` if creation succeeds, otherwise an error string.
    pub fn new(topic_name: &str) -> Result<Self, String> {
        let datatype = T::datatype();
        let publisher = Publisher::new(topic_name, datatype)?;

        Ok(Self {
            publisher,
            _phantom: PhantomData,
        })
    }

    /// Publishes a message of type `T` to the topic.
    ///
    /// This method serializes the message using `T::to_bytes()` and
    /// sends the binary buffer using the underlying eCAL publisher.
    ///
    /// # Arguments
    ///
    /// * `message` - A reference to the message to publish.
    pub fn send(&self, message: &T) {
        let bytes = message.to_bytes();
        self.publisher.send(&bytes);
    }
}
