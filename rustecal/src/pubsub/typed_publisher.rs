use crate::pubsub::publisher::Publisher;
use crate::pubsub::types::{DataTypeInfo, TopicId};
use std::marker::PhantomData;

/// Trait for types that can be published via [`TypedPublisher`].
///
/// Implement this trait for any message type `T` that should be serialized and sent
/// using the typed publisher API.
///
/// # Required Methods
///
/// - [`datatype`]: Returns metadata including encoding, type name, and optional descriptor.
/// - [`to_bytes`]: Serializes the message to a binary format.
///
/// This trait must be implemented by any message type `T` that should be
/// transmitted via the typed publisher API.
///
/// # Required Methods
///
/// - [`datatype`]: Returns metadata describing the encoding, type name,
///   and descriptor (e.g., Protobuf schema).
/// - [`to_bytes`]: Serializes the message instance into a binary buffer.
pub trait PublisherMessage {
    /// Returns topic metadata for this type.
    fn datatype() -> DataTypeInfo;

    /// Serializes the message into a byte buffer for transmission.
    fn to_bytes(&self) -> Vec<u8>;
}

/// Type-safe, high-level wrapper around an eCAL publisher.
///
/// This struct is generic over a type `T` implementing [`PublisherMessage`],
/// and ensures safe, typed interaction with the eCAL publish-subscribe system.
///
/// Internally, it wraps a raw [`Publisher`] instance and provides additional
/// features like automatic serialization and type-safe publishing.
///
/// # Example
///
/// ```no_run
/// use rustecal::TypedPublisher;
/// use rustecal_types_string::StringMessage;
///
/// let pub_ = TypedPublisher::<StringMessage>::new("example").unwrap();
/// pub_.send(&StringMessage("Hello World!".into()));
/// ```
pub struct TypedPublisher<T: PublisherMessage> {
    publisher: Publisher,
    _phantom: PhantomData<T>,
}

impl<T: PublisherMessage> TypedPublisher<T> {
    /// Creates a new typed publisher for the given topic name.
    ///
    /// # Arguments
    ///
    /// * `topic_name` - The topic name used for publishing messages.
    ///
    /// # Errors
    ///
    /// Returns a `String` if the underlying eCAL publisher cannot be created.
    pub fn new(topic_name: &str) -> Result<Self, String> {
        let datatype = T::datatype();
        let publisher = Publisher::new(topic_name, datatype)?;

        Ok(Self {
            publisher,
            _phantom: PhantomData,
        })
    }

    /// Sends a typed message to all connected subscribers.
    ///
    /// The message is serialized using the [`PublisherMessage::to_bytes`] method.
    ///
    /// # Arguments
    ///
    /// * `message` - The message of type `T` to send.
    pub fn send(&self, message: &T) {
        let bytes = message.to_bytes();
        self.publisher.send(&bytes);
    }

    /// Sends a message with a custom timestamp (in microseconds).
    ///
    /// # Arguments
    ///
    /// * `message` - The message to send.
    /// * `timestamp` - Custom timestamp to associate with the message.
    pub fn send_with_timestamp(&self, message: &T, timestamp: i64) {
        let bytes = message.to_bytes();
        self.publisher.send_with_timestamp(&bytes, timestamp);
    }

    /// Returns the number of currently connected subscribers.
    pub fn get_subscriber_count(&self) -> usize {
        self.publisher.get_subscriber_count()
    }

    /// Returns the name of the topic this publisher is bound to.
    pub fn get_topic_name(&self) -> Option<String> {
        self.publisher.get_topic_name()
    }

    /// Returns the topic ID (as seen by the eCAL system).
    pub fn get_topic_id(&self) -> Option<TopicId> {
        self.publisher.get_topic_id()
    }

    /// Returns the declared message metadata for this publisher.
    pub fn get_data_type_information(&self) -> Option<DataTypeInfo> {
        self.publisher.get_data_type_information()
    }
}
