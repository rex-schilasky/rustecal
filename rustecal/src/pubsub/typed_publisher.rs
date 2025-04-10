use crate::pubsub::publisher::Publisher;
use crate::pubsub::types::DataTypeInfo;
use std::marker::PhantomData;

/// Trait for any type that can be published via `TypedPublisher`
pub trait PublisherMessage {
    fn datatype() -> DataTypeInfo;
    fn to_bytes(&self) -> Vec<u8>;
}

/// High-level type-safe publisher wrapper
pub struct TypedPublisher<T: PublisherMessage> {
    publisher: Publisher,
    _phantom: PhantomData<T>,
}

impl<T: PublisherMessage> TypedPublisher<T> {
    pub fn new(topic: &str) -> Result<Self, String> {
        let datatype = T::datatype();
        let publisher = Publisher::new(topic, datatype)?;

        Ok(Self {
            publisher,
            _phantom: PhantomData,
        })
    }

    pub fn send(&self, message: &T) {
        let bytes = message.to_bytes();
        self.publisher.send(&bytes);
    }
}
