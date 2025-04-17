//! Common eCAL types shared across pubsub and service layers.

use std::ffi::CStr;
use std::os::raw::c_char;

/// Represents a globally unique entity in eCAL.
#[derive(Debug, Clone)]
pub struct EntityId {
    pub entity_id: u64,
    pub process_id: i32,
    pub host_name: String,
}

impl From<rustecal_sys::eCAL_SEntityId> for EntityId {
    fn from(raw: rustecal_sys::eCAL_SEntityId) -> Self {
        Self {
            entity_id: raw.entity_id,
            process_id: raw.process_id,
            host_name: cstr_to_string(raw.host_name),
        }
    }
}

/// Rust-safe representation of `eCAL_SDataTypeInformation`.
#[derive(Debug, Clone)]
pub struct DataTypeInfo {
    pub type_name: String,
    pub encoding: String,
    pub descriptor: Vec<u8>,
}

impl From<rustecal_sys::eCAL_SDataTypeInformation> for DataTypeInfo {
    fn from(info: rustecal_sys::eCAL_SDataTypeInformation) -> Self {
        let type_name = cstr_to_string(info.name);
        let encoding = cstr_to_string(info.encoding);
        let descriptor = if info.descriptor.is_null() || info.descriptor_length == 0 {
            vec![]
        } else {
            unsafe {
                std::slice::from_raw_parts(info.descriptor as *const u8, info.descriptor_length)
                    .to_vec()
            }
        };

        Self {
            type_name,
            encoding,
            descriptor,
        }
    }
}

/// Rust-safe representation of `eCAL_SVersion`.
#[derive(Debug, Clone)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

impl From<rustecal_sys::eCAL_SVersion> for Version {
    fn from(raw: rustecal_sys::eCAL_SVersion) -> Self {
        Self {
            major: raw.major,
            minor: raw.minor,
            patch: raw.patch,
        }
    }
}

/// Helper to safely convert null-terminated C strings.
fn cstr_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
    }
}
