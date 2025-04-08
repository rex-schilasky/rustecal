use crate::pubsub::types::DataTypeInfo;
use rustecal_sys::*;
use std::ffi::CString;
use std::ptr;

pub struct Publisher {
    handle: *mut eCAL_Publisher,
    _encoding: CString,
    _type_name: CString,
    _descriptor: Vec<u8>,
}

impl Publisher {
    pub fn new(topic_name: &str, info: DataTypeInfo) -> Result<Self, String> {
        let c_topic = CString::new(topic_name).map_err(|_| "Invalid topic name")?;
        let c_encoding = CString::new(info.encoding).map_err(|_| "Invalid encoding string")?;
        let c_type_name = CString::new(info.type_name).map_err(|_| "Invalid type name")?;

        let descriptor_ptr = if info.descriptor.is_empty() {
            ptr::null()
        } else {
            info.descriptor.as_ptr() as *const std::ffi::c_void
        };

        let data_type_info = eCAL_SDataTypeInformation {
            encoding: c_encoding.as_ptr(),
            name: c_type_name.as_ptr(),
            descriptor: descriptor_ptr,
            descriptor_length: info.descriptor.len(),
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
                _descriptor: info.descriptor,
            })
        }
    }

    pub fn send(&self, data: &[u8]) -> Result<(), String> {
        let result = unsafe {
            eCAL_Publisher_Send(
                self.handle,
                data.as_ptr() as *const _,
                data.len(),
                ptr::null(),
            )
        };

        // TODO: we need to fix eCAL_Publisher_Send it seems to return 1 if succeeded
        if result == 1 {
            Ok(())
        } else {
            Err(format!("Send failed with code {}", result))
        }
    }

    pub fn subscriber_count(&self) -> usize {
        unsafe { eCAL_Publisher_GetSubscriberCount(self.handle) }
    }
}

impl Drop for Publisher {
    fn drop(&mut self) {
        unsafe {
            eCAL_Publisher_Delete(self.handle);
        }
    }
}
