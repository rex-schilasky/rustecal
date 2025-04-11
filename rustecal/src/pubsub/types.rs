use std::ffi::c_void;
use std::os::raw::c_char;

/// Rust-side representation of message metadata used in eCAL.
///
/// This structure is used to describe the encoding, type name, and optional
/// schema (e.g. protobuf descriptor) associated with a message.
///
/// It is passed to publishers and subscribers to ensure that the message
/// type and encoding are correctly interpreted across systems.
#[derive(Debug, Clone)]
pub struct DataTypeInfo {
    /// Encoding format (e.g. "proto", "string", "raw").
    pub encoding: String,

    /// Logical or fully-qualified type name (e.g. `pb.MyType`).
    pub type_name: String,

    /// Optional binary descriptor for the message schema (e.g. protobuf descriptor).
    pub descriptor: Vec<u8>,
}

/// An identifier used by eCAL to distinguish topics.
///
/// This includes the internal UUID-style ID and the topic name as a raw C string.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TopicId {
    /// Unique entity identifier assigned to the topic.
    pub topic_id: EntityId,

    /// C string pointer to the topic name.
    pub topic_name: *const c_char,
}

/// A generic 128-bit UUID-style identifier used across eCAL entities.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EntityId {
    /// Raw 128-bit identifier value.
    pub id: [u8; 16],
}

/// Enum of publisher events that may trigger callbacks.
///
/// Events are reported through `PublisherEventCallbackData`.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PublisherEvent {
    /// No event occurred.
    None = 0,

    /// A subscriber has connected.
    Connected = 1,

    /// A subscriber has disconnected.
    Disconnected = 2,

    /// A message was dropped.
    Dropped = 3,
}

/// Data structure provided to publisher event callbacks.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PublisherEventCallbackData {
    /// The type of event that occurred.
    pub event: PublisherEvent,

    /// Timestamp of the event (microseconds).
    pub time: i64,

    /// Logical clock value at time of event.
    pub clock: i64,

    /// State code (implementation-specific).
    pub state: i32,
}

/// Enum of subscriber events that may trigger callbacks.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SubscriberEvent {
    /// No event occurred.
    None = 0,

    /// A publisher has connected.
    Connected = 1,

    /// A publisher has disconnected.
    Disconnected = 2,

    /// A message was dropped.
    Dropped = 3,
}

/// Data structure provided to subscriber event callbacks.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SubscriberEventCallbackData {
    /// The type of event that occurred.
    pub event: SubscriberEvent,

    /// Timestamp of the event (microseconds).
    pub time: i64,

    /// Logical clock value at time of event.
    pub clock: i64,

    /// State code (implementation-specific).
    pub state: i32,
}

/// Represents data received by a subscriber callback.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ReceiveCallbackData {
    /// Pointer to the received message payload.
    pub buffer: *const c_void,

    /// Size of the payload in bytes.
    pub buffer_size: usize,

    /// Timestamp of when the message was sent (microseconds).
    pub send_timestamp: i64,

    /// Clock value associated with the send event.
    pub send_clock: i64,
}

/// Raw FFI version of [`DataTypeInfo`] used in C interop.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiDataTypeInfo {
    /// Pointer to encoding C string.
    pub encoding: *const c_char,

    /// Pointer to type name C string.
    pub name: *const c_char,

    /// Pointer to descriptor buffer.
    pub descriptor: *const c_void,

    /// Length of the descriptor buffer.
    pub descriptor_length: usize,
}

/// Raw FFI version of [`TopicId`] used in callbacks.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiTopicId {
    pub topic_id: EntityId,
    pub topic_name: *const c_char,
}

/// Raw FFI version of [`ReceiveCallbackData`] used in callbacks.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiReceiveCallbackData {
    pub buffer: *const c_void,
    pub buffer_size: usize,
    pub send_timestamp: i64,
    pub send_clock: i64,
}
