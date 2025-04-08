// src/pubsub/types.rs

use std::ffi::c_void;
use std::os::raw::c_char;

/// Safe Rust wrapper type for describing data type info
#[derive(Debug, Clone)]
pub struct DataTypeInfo {
    pub encoding: String,
    pub type_name: String,
    pub descriptor: Vec<u8>,
}

/// FFI-safe representation of an eCAL topic ID
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TopicId {
    pub topic_id: EntityId,
    pub topic_name: *const c_char,
}

/// FFI-safe representation of a generic eCAL entity ID
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EntityId {
    pub id: [u8; 16],
}

/// Event types for publishers
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PublisherEvent {
    None = 0,
    Connected = 1,
    Disconnected = 2,
    Dropped = 3,
}

/// Publisher event callback data
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PublisherEventCallbackData {
    pub event: PublisherEvent,
    pub time: i64,
    pub clock: i64,
    pub state: i32,
}

/// Subscriber event types
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SubscriberEvent {
    None = 0,
    Connected = 1,
    Disconnected = 2,
    Dropped = 3,
}

/// Subscriber event callback data
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SubscriberEventCallbackData {
    pub event: SubscriberEvent,
    pub time: i64,
    pub clock: i64,
    pub state: i32,
}

/// Receive callback data for subscribers
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ReceiveCallbackData {
    pub buffer: *const c_void,
    pub buffer_size: usize,
    pub send_timestamp: i64,
    pub send_clock: i64,
}