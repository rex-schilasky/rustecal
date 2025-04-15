use rustecal_sys::*;
use crate::pubsub::types::{DataTypeInfo, TopicId};
use std::ffi::{CStr, CString};
use std::ptr;

/// A safe and ergonomic wrapper around the eCAL C publisher API.
///
/// This struct provides a high-level interface for sending serialized messages to
/// a topic using eCAL. It manages the lifecycle of the underlying eCAL publisher handle
/// and exposes convenient methods to access metadata and send data.
pub struct Publisher {
    handle: *mut eCAL_Publisher,
    _encoding: CString,
    _type_name: CString,
    _descriptor: Vec<u8>,
}

impl Publisher {
    /// Creates a new publisher for the given topic with type metadata.
    ///
    /// # Arguments
    ///
    /// * `topic_name` - The topic to publish messages on.
    /// * `data_type` - The encoding, type name, and optional descriptor for the topic.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Publisher)` if creation succeeds, or `Err` with a message if it fails.
    pub fn new(topic_name: &str, data_type: DataTypeInfo) -> Result<Self, String> {
        let c_topic = CString::new(topic_name).map_err(|_| "Invalid topic name")?;
        let c_encoding = CString::new(data_type.encoding).map_err(|_| "Invalid encoding string")?;
        let c_type_name = CString::new(data_type.type_name).map_err(|_| "Invalid type name")?;

        let descriptor_ptr = if data_type.descriptor.is_empty() {
            ptr::null()
        } else {
            data_type.descriptor.as_ptr() as *const std::ffi::c_void
        };

        let data_type_info = eCAL_SDataTypeInformation {
            encoding: c_encoding.as_ptr(),
            name: c_type_name.as_ptr(),
            descriptor: descriptor_ptr,
            descriptor_length: data_type.descriptor.len(),
        };

        let handle = unsafe {
            eCAL_Publisher_New(c_topic.as_ptr(), &data_type_info, None, ptr::null())
        };

        if handle.is_null() {
            Err("Failed to create eCAL_Publisher".into())
        } else {
            Ok(Self {
                handle,
                _encoding: c_encoding,
                _type_name: c_type_name,
                _descriptor: data_type.descriptor,
            })
        }
    }

    /// Sends a serialized message to all connected subscribers.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte buffer containing the serialized message payload.
    ///
    /// # Returns
    ///
    /// `1` on success, `0` on failure.
    pub fn send(&self, data: &[u8]) -> i32 {
        unsafe {
            eCAL_Publisher_Send(
                self.handle,
                data.as_ptr() as *const _,
                data.len(),
                ptr::null(),
            )
        }
    }

    /// Sends a serialized message with a custom timestamp.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte buffer containing the message.
    /// * `timestamp` - Timestamp in microseconds (use `-1` to let eCAL determine the time).
    ///
    /// # Returns
    ///
    /// `1` on success, `0` on failure.
    pub fn send_with_timestamp(&self, data: &[u8], timestamp: i64) -> i32 {
        unsafe {
            eCAL_Publisher_Send(
                self.handle,
                data.as_ptr() as *const _,
                data.len(),
                &timestamp as *const _ as *const _,
            )
        }
    }

    /// Returns the number of currently connected subscribers.
    pub fn get_subscriber_count(&self) -> usize {
        unsafe { eCAL_Publisher_GetSubscriberCount(self.handle) }
    }

    /// Retrieves the name of the topic being published.
    ///
    /// # Returns
    ///
    /// The topic name as a `String`, or `None` if unavailable.
    pub fn get_topic_name(&self) -> Option<String> {
        unsafe {
            let raw = eCAL_Publisher_GetTopicName(self.handle);
            if raw.is_null() {
                None
            } else {
                Some(CStr::from_ptr(raw).to_string_lossy().into_owned())
            }
        }
    }

    /// Retrieves the internal eCAL topic ID for this publisher.
    ///
    /// # Returns
    ///
    /// A [`TopicId`] struct, or `None` if the information is unavailable.
    pub fn get_topic_id(&self) -> Option<TopicId> {
        unsafe {
            let raw = eCAL_Publisher_GetTopicId(self.handle);
            if raw.is_null() {
                None
            } else {
                Some(*(raw as *const TopicId))
            }
        }
    }

    /// Retrieves the declared data type information for the publisher.
    ///
    /// # Returns
    ///
    /// A [`DataTypeInfo`] object containing encoding, type name, and descriptor,
    /// or `None` if the metadata is unavailable.
    pub fn get_data_type_information(&self) -> Option<DataTypeInfo> {
        unsafe {
            let raw = eCAL_Publisher_GetDataTypeInformation(self.handle);
            if raw.is_null() {
                return None;
            }

            let info = &*raw;

            let encoding = if info.encoding.is_null() {
                String::new()
            } else {
                CStr::from_ptr(info.encoding).to_string_lossy().into_owned()
            };

            let type_name = if info.name.is_null() {
                String::new()
            } else {
                CStr::from_ptr(info.name).to_string_lossy().into_owned()
            };

            let descriptor = if info.descriptor.is_null() || info.descriptor_length == 0 {
                vec![]
            } else {
                std::slice::from_raw_parts(info.descriptor as *const u8, info.descriptor_length).to_vec()
            };

            Some(DataTypeInfo {
                encoding,
                type_name,
                descriptor,
            })
        }
    }
}

impl Drop for Publisher {
    /// Cleans up the underlying eCAL publisher resource.
    fn drop(&mut self) {
        unsafe {
            eCAL_Publisher_Delete(self.handle);
        }
    }
}
