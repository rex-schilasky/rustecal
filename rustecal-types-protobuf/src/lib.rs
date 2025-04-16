//! Typed Protobuf message support for `rustecal`.
//!
//! This crate provides a wrapper around `prost::Message`-based types, allowing
//! seamless integration with `TypedPublisher` and `TypedSubscriber` from the `rustecal` API.
//!
//! ## Usage
//!
//! To publish or subscribe to Protobuf messages, wrap your message type in `ProtobufMessage<T>`
//! and ensure the inner type implements both `prost::Message` and `IsProtobufType`.
//!
//! ```rust
//! use rustecal_types_protobuf::{ProtobufMessage, IsProtobufType};
//! use my_proto::MyMessage;
//!
//! impl IsProtobufType for MyMessage {}
//!
//! let msg = ProtobufMessage(MyMessage::default());
//! ```

use prost::Message;
use rustecal::pubsub::{PublisherMessage, SubscriberMessage};
use rustecal::ecal::types::DataTypeInfo;

/// Marker trait to opt-in a Protobuf type for use with eCAL.
///
/// This trait must be implemented for any `prost::Message` you wish to use
/// with `ProtobufMessage<T>`. It provides a type-level opt-in mechanism
/// to ensure users are aware of what's being exposed to eCAL.
pub trait IsProtobufType {}

/// Wrapper around a `prost::Message` type to enable typed publishing and subscription.
///
/// This is the type that should be used with `TypedPublisher` and `TypedSubscriber`
/// for Protobuf messages.
///
/// ```rust
/// use rustecal_types_protobuf::{ProtobufMessage, IsProtobufType};
/// use my_proto::MyMessage;
///
/// impl IsProtobufType for MyMessage {}
///
/// let wrapped = ProtobufMessage(MyMessage::default());
/// ```
#[derive(Debug, Clone)]
pub struct ProtobufMessage<T>(pub T);

impl<T> SubscriberMessage for ProtobufMessage<T>
where
    T: Message + Default + IsProtobufType,
{
    /// Returns metadata used by eCAL to describe the Protobuf type.
    ///
    /// This includes:
    /// - `proto` as encoding
    /// - the Rust type name
    /// - an optional descriptor (currently empty)
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: "proto".to_string(),
            type_name: std::any::type_name::<T>().to_string(),
            descriptor: vec![], // descriptor injection planned
        }
    }

    /// Decodes a Protobuf message from bytes.
    ///
    /// # Returns
    /// - `Some(ProtobufMessage<T>)` on success
    /// - `None` if decoding fails
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        T::decode(bytes).ok().map(ProtobufMessage)
    }
}

impl<T> PublisherMessage for ProtobufMessage<T>
where
    T: Message + Default + IsProtobufType,
{
    /// Returns the same datatype information as [`SubscriberMessage`] implementation.
    fn datatype() -> DataTypeInfo {
        <ProtobufMessage<T> as SubscriberMessage>::datatype()
    }

    /// Encodes the message to a byte buffer.
    ///
    /// # Panics
    /// Will panic if `prost::Message::encode` fails (should never panic for valid messages).
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.0.encoded_len());
        self.0
            .encode(&mut buf)
            .expect("Failed to encode protobuf message");
        buf
    }
}
