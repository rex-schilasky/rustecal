//! Rust-safe wrapper for `eCAL_Logging_SLogMessage`.
//!
//! This type represents individual log entries emitted by eCAL.

use crate::log_level::LogLevel;
use std::ffi::CStr;
use std::os::raw::c_char;

/// Represents a single log message emitted by eCAL.
#[derive(Debug, Clone)]
pub struct LogMessage {
    pub level: LogLevel,
    pub timestamp: i64,
    pub host_name: String,
    pub process_name: String,
    pub process_id: i32,
    pub thread_name: String,
    pub content: String,
}

impl From<rustecal_sys::eCAL_Logging_SLogMessage> for LogMessage {
    fn from(raw: rustecal_sys::eCAL_Logging_SLogMessage) -> Self {
        Self {
            level: LogLevel::from(raw.level),
            timestamp: raw.time,
            host_name: cstr_to_string(raw.host_name),
            process_name: cstr_to_string(raw.process_name),
            process_id: raw.process_id,
            thread_name: cstr_to_string(raw.unit_name), // or rename field to unit_name
            content: cstr_to_string(raw.content),
        }
    }
}

/// Converts a C string pointer to a Rust `String`.
fn cstr_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
    }
}
