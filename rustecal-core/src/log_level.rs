//! eCAL logging severity levels.
//!
//! This mirrors the C enum `eCAL_Logging_eLogLevel` and is used in logging APIs.

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    None    = 0,
    Fatal   = 1,
    Error   = 2,
    Warning = 3,
    Info    = 4,
    Debug   = 5,
    Verbose = 6,
}

impl From<i32> for LogLevel {
    fn from(value: i32) -> Self {
        match value {
            1 => LogLevel::Fatal,
            2 => LogLevel::Error,
            3 => LogLevel::Warning,
            4 => LogLevel::Info,
            5 => LogLevel::Debug,
            6 => LogLevel::Verbose,
            _ => LogLevel::None,
        }
    }
}

impl From<LogLevel> for i32 {
    fn from(level: LogLevel) -> Self {
        level as i32
    }
}
