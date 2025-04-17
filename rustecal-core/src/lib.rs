//! rustecal-core: eCAL initialization, shared components, logging, and core types.
//!
//! This crate provides safe Rust wrappers for the core system of eCAL,
//! including runtime lifecycle management, logging, and reusable data structures.

pub mod core;
pub mod components;
pub mod types;
pub mod log;
pub mod log_level;
pub mod core_types;

// Re‑exports for ergonomic access:
pub use core::Ecal;
pub use components::EcalComponents;
pub use log::Log;
pub use log_level::LogLevel;
pub use core_types::logging::LogMessage;
