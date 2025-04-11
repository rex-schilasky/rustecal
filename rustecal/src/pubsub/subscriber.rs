use rustecal_sys::*;
use crate::pubsub::types::{DataTypeInfo, TopicId};
use std::ffi::{CString, CStr, c_void};
use std::ptr;

/// A safe wrapper around the eCAL C subscriber API.
///
/// This struct allows subscribing to a named topic using a user-provided
/// callback and receiving deserialized messages of a specific data type.
pub struct Subscriber {
    handle: *mut eCAL_Subscriber,
    _encoding: CString,
    _type_name: CString,
    _descriptor: Vec<u8>,
}

impl Subscriber {
    /// Creates a new subscriber and assigns a receive callback.
    ///
    /// # Arguments
    ///
    /// * `topic_name` - The name of the topic to subscribe to.
    /// * `data_type` - Metadata describing the expected message format.
    /// * `callback` - A raw extern "C" callback function to invoke on reception.
    /// * `_user_data` - Reserved for future use (currently ignored).
    ///
    /// # Returns
    ///
    /// Returns `Ok(Subscriber)` on success or `Err(String)` on failure.
    pub fn new(
        topic_name: &str,
        data_type: DataTypeInfo,
        callback: extern "C" fn(
            *const eCAL_STopicId,
            *const eCAL_SDataTypeInformation,
            *const eCAL_SReceiveCallbackData,
            *mut c_void,
        ),
        _user_data: *mut c_void, // Ignored
    ) -> Result<Self, String> {
        let c_topic = CString::new(topic_name).map_err(|_| "Invalid topic name")?;
        let c_encoding = CString::new(data_type.encoding).map_err(|_| "Invalid encoding")?;
        let c_type_name = CString::new(data_type.type_name).map_err(|_| "Invalid type name")?;

        let descriptor_ptr = if data_type.descriptor.is_empty() {
            ptr::null()
        } else {
            data_type.descriptor.as_ptr() as *const c_void
        };

        let data_type_info = eCAL_SDataTypeInformation {
            encoding: c_encoding.as_ptr(),
            name: c_type_name.as_ptr(),
            descriptor: descriptor_ptr,
            descriptor_length: data_type.descriptor.len(),
        };

        let handle = unsafe {
            eCAL_Subscriber_New(
                c_topic.as_ptr(),
                &data_type_info,
                None,
                ptr::null(),
            )
        };

        if handle.is_null() {
            return Err("Failed to create eCAL_Subscriber".into());
        }

        let result = unsafe {
            eCAL_Subscriber_SetReceiveCallback(
                handle,
                Some(callback),
                ptr::null_mut(), // Always set to null
            )
        };

        if result != 0 {
            return Err("Failed to set receive callback".into());
        }

        Ok(Self {
            handle,
            _encoding: c_encoding,
            _type_name: c_type_name,
            _descriptor: data_type.descriptor,
        })
    }

    /// Returns the raw C pointer to the eCAL subscriber handle.
    pub fn raw_handle(&self) -> *mut eCAL_Subscriber {
        self.handle
    }

    /// Returns the number of currently connected publishers.
    pub fn get_publisher_count(&self) -> usize {
        unsafe { eCAL_Subscriber_GetPublisherCount(self.handle) }
    }

    /// Retrieves the topic name this subscriber is connected to.
    ///
    /// # Returns
    ///
    /// The topic name as a `String`, or `None` if retrieval fails.
    pub fn get_topic_name(&self) -> Option<String> {
        unsafe {
            let raw = eCAL_Subscriber_GetTopicName(self.handle);
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
            let raw = eCAL_Subscriber_GetTopicId(self.handle);
            if raw.is_null() {
                None
            } else {
                Some(*(raw as *const TopicId))
            }
        }
    }

    /// Returns the declared data type metadata for this subscriber.
    ///
    /// This includes the declared encoding, type name, and optionally the
    /// descriptor (e.g. Protobuf schema bytes).
    pub fn get_data_type_information(&self) -> Option<DataTypeInfo> {
        unsafe {
            let raw = eCAL_Subscriber_GetDataTypeInformation(self.handle);
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

impl Drop for Subscriber {
    /// Removes the receive callback and deletes the underlying eCAL handle.
    fn drop(&mut self) {
        unsafe {
            eCAL_Subscriber_RemoveReceiveCallback(self.handle);
            eCAL_Subscriber_Delete(self.handle);
        }
    }
}
