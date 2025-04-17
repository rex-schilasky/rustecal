//! Core logging functions for emitting messages through eCAL.
//!
//! This module wraps the C API from `ecal_c/log.h` and provides access to
//! logging at various severity levels, as well as retrieval of current log entries.

use crate::core_types::logging::LogMessage;
use crate::log_level::LogLevel;

use std::ffi::CString;
use std::ptr;
use std::slice;

/// Provides logging functions to emit and retrieve messages via the eCAL runtime.
pub struct Log;

impl Log {
    /// Emits a message to the eCAL logging system with a specified severity.
    ///
    /// # Arguments
    ///
    /// * `level` - Log severity (e.g., [`LogLevel::Info`], [`LogLevel::Error`])
    /// * `message` - The log content to emit
    pub fn log(level: LogLevel, message: &str) {
        let cstr = CString::new(message)
            .unwrap_or_else(|_| CString::new("<invalid UTF-8>").unwrap());

        unsafe {
            rustecal_sys::eCAL_Logging_Log(level.into(), cstr.as_ptr());
        }
    }

    /// Fetches all current log messages stored in the eCAL runtime.
    ///
    /// This function uses the C API `eCAL_Logging_GetLogging` to retrieve
    /// structured log messages and ensures the memory is released with `eCAL_Free`.
    ///
    /// # Returns
    ///
    /// A vector of [`LogMessage`] entries, or an empty vector if retrieval failed.
    pub fn get_logging() -> Vec<LogMessage> {
        let mut raw_ptr: *mut rustecal_sys::eCAL_Logging_SLogging = ptr::null_mut();

        let success = unsafe { rustecal_sys::eCAL_Logging_GetLogging(&mut raw_ptr) };

        if success != 0 || raw_ptr.is_null() {
            return vec![];
        }

        let logging = unsafe { &*raw_ptr };
        let raw_messages = logging.log_messages;
        let len = logging.log_messages_length;

        let logs = unsafe {
            slice::from_raw_parts(raw_messages, len)
                .iter()
                .map(|msg| LogMessage::from(*msg))
                .collect()
        };

        unsafe {
            rustecal_sys::eCAL_Free(raw_ptr as *mut _);
        }

        logs
    }
}
