use crate::pubsub::types::{DataTypeInfo, TopicId};
use rustecal_sys::*;
use std::ffi::{CString, CStr};
use std::ptr;

/// A safe wrapper around the eCAL C publisher API.
///
/// This struct handles publishing serialized messages via eCAL, using
/// strongly typed metadata (`DataTypeInfo`) and managing the lifecycle of
/// the underlying C handle.
pub struct Publisher {
    handle: *mut eCAL_Publisher,
    _encoding: CString,
    _type_name: CString,
    _descriptor: Vec<u8>,
}

impl Publisher {
    /// Creates a new publisher for the given topic and message type.
    ///
    /// # Arguments
    ///
    /// * `topic_name` - The name of the topic to publish to.
    /// * `data_type` - Metadata describing the message type (encoding, type name, optional descriptor).
    ///
    /// # Returns
    ///
    /// A new `Publisher` instance if successful, or an error string if creation fails.
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
            eCAL_Publisher_New(
                c_topic.as_ptr(),
                &data_type_info,
                None,
                ptr::null(),
            )
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

    /// Sends a message to all connected subscribers.
    ///
    /// # Arguments
    ///
    /// * `data` - Message buffer.
    ///
    /// # Returns
    ///
    /// An integer status (0 = failure, 1 = success).
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

    /// Sends a message to all subscribers with an explicit send timestamp.
    ///
    /// # Arguments
    ///
    /// * `data` - Message buffer.
    /// * `timestamp` - Time in microseconds (use -1 for eCAL system time).
    ///
    /// # Returns
    ///
    /// An integer status (0 = failure, 1 = success).
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

    /// Retrieves the topic name this publisher is connected to.
    ///
    /// # Returns
    ///
    /// The topic name as a `String`, or `None` if retrieval fails.
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

    /// Retrieves the topic ID used internally by eCAL.
    ///
    /// # Returns
    ///
    /// A `TopicId` struct or `None` if unavailable.
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

    /// Returns the declared data type metadata for this publisher.
    ///
    /// This includes the declared encoding, type name, and optionally the
    /// descriptor (e.g. Protobuf schema bytes).
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
    /// Releases the underlying eCAL publisher when dropped.
    fn drop(&mut self) {
        unsafe {
            eCAL_Publisher_Delete(self.handle);
        }
    }
}
