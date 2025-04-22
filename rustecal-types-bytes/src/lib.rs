//! # rustecal-types-bytes
//!
//! Provides support for sending and receiving raw binary messages (`Vec<u8>`) with rustecal.
//!
//! ## Example
//! ```rust
//! use rustecal_types_bytes::BytesMessage;
//! let msg = BytesMessage(vec![1, 2, 3, 4]);
//! ```

use rustecal_core::types::DataTypeInfo;
use rustecal_pubsub::typed_publisher::PublisherMessage;
use rustecal_pubsub::typed_subscriber::SubscriberMessage;

/// A wrapper for raw binary data transmitted via eCAL.
///
/// This message type is ideal for non-structured byte payloads such as images,
/// serialized custom formats, or arbitrary buffers.
///
/// Implements both [`PublisherMessage`] and [`SubscriberMessage`] to support
/// bidirectional pub/sub use.
pub struct BytesMessage(pub Vec<u8>);

impl SubscriberMessage for BytesMessage {
    /// Returns metadata describing the message encoding and type.
    ///
    /// Encoding is `"raw"`, type name is `"bytes"`, and no descriptor is included.
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: "raw".into(),
            type_name: "bytes".into(),
            descriptor: vec![],
        }
    }

    /// Creates a `BytesMessage` from a raw byte slice.
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Some(BytesMessage(bytes.to_vec()))
    }
}

impl PublisherMessage for BytesMessage {
    /// Reuses the `SubscriberMessage::datatype()` implementation.
    fn datatype() -> DataTypeInfo {
        <BytesMessage as SubscriberMessage>::datatype()
    }

    /// Converts the internal byte vector into a byte slice for sending.
    fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}
