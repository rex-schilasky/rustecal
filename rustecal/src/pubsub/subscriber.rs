// src/pubsub/subscriber.rs

use crate::pubsub::types::{DataTypeInfo, FfiDataTypeInfo, FfiReceiveCallbackData, FfiTopicId};
use rustecal_sys::*;
use std::ffi::CString;
use std::ptr;
use std::ffi::c_void;

pub struct Subscriber {
    handle: *mut eCAL_Subscriber,
    _encoding: CString,
    _type_name: CString,
    _descriptor: Vec<u8>,
}

impl Subscriber {
    pub fn new(
        topic_name: &str,
        data_type: DataTypeInfo,
        callback: extern "C" fn(
            *const FfiTopicId,
            *const FfiDataTypeInfo,
            *const FfiReceiveCallbackData,
            *mut c_void,
        ),
        user_data: *mut c_void,
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
            eCAL_Subscriber_SetReceiveCallback(handle, Some(std::mem::transmute(callback)), user_data)
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
}

impl Drop for Subscriber {
    fn drop(&mut self) {
        unsafe {
            eCAL_Subscriber_RemoveReceiveCallback(self.handle);
            eCAL_Subscriber_Delete(self.handle);
        }
    }
}