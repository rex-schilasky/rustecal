//! # rustecal-types-string
//!
//! This crate provides support for sending and receiving UTF-8 string messages
//! using the `rustecal` typed publisher and subscriber APIs.
//!
//! It defines a wrapper type [`StringMessage`] that implements the necessary traits
//! [`PublisherMessage`] and [`SubscriberMessage`] for type-safe usage with
//! [`TypedPublisher`] and [`TypedSubscriber`] respectively.

use rustecal_core::types::DataTypeInfo;
use rustecal_pubsub::typed_publisher::PublisherMessage;
use rustecal_pubsub::typed_subscriber::SubscriberMessage;
use std::str;

/// A wrapper for UTF-8 string messages used with typed eCAL pub/sub.
///
/// This type allows sending and receiving strings through the
/// `TypedPublisher<StringMessage>` and `TypedSubscriber<StringMessage>` APIs.
pub struct StringMessage(pub String);

impl SubscriberMessage for StringMessage {
    /// Returns metadata describing this message type (`utf-8` encoded string).
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: "utf-8".to_string(),
            type_name: "string".to_string(),
            descriptor: vec![],
        }
    }

    /// Attempts to decode a UTF-8 string from a byte buffer.
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        str::from_utf8(bytes).ok().map(|s| StringMessage(s.to_string()))
    }
}

impl PublisherMessage for StringMessage {
    /// Returns the same metadata as [`SubscriberMessage::datatype`].
    fn datatype() -> DataTypeInfo {
        <StringMessage as SubscriberMessage>::datatype()
    }

    /// Serializes the string into a byte buffer.
    fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}
