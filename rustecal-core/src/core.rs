//! Core lifecycle control for the eCAL runtime.
//!
//! This module provides safe Rust wrappers around initializing and finalizing
//! the eCAL communication system, as well as querying its state and version.
//!
//! The main entry point is the [`Ecal`] struct which provides:
//! - [`Ecal::initialize`] to start the middleware
//! - [`Ecal::finalize`] to shut it down
//! - [`Ecal::ok`] to query if eCAL is currently running
//! - [`Ecal::is_initialized`] and [`Ecal::is_component_initialized`] for introspection
//! - [`Ecal::version_string`], [`Ecal::version_date_string`] and [`Ecal::version_struct`] for version info
//!
//! Typically, you will call [`Ecal::initialize`] once at the beginning of your
//! application and [`Ecal::finalize`] at shutdown.

use std::ffi::{CStr, CString};
use crate::components::EcalComponents;
use crate::types::Version;

/// Provides access to the core initialization, shutdown, and state-checking functions of eCAL.
pub struct Ecal;

impl Ecal {
    /// Initializes the eCAL runtime system.
    ///
    /// This function must be called before using any publisher, subscriber, or service functionality.
    ///
    /// # Arguments
    ///
    /// * `unit_name` - Optional name to identify this process in eCAL (e.g. in monitoring).
    /// * `components` - Bitmask of which subsystems (e.g. pub/sub, monitoring) to enable.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or `Err(code)` with a non-zero error code.
    pub fn initialize(unit_name: Option<&str>, components: EcalComponents) -> Result<(), i32> {
        let cstr = unit_name.map(|s| CString::new(s).unwrap());
        let ptr = cstr.as_ref().map_or(std::ptr::null(), |c| c.as_ptr());

        let result = unsafe {
            rustecal_sys::eCAL_Initialize(ptr, &components.bits(), std::ptr::null())
        };

        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    /// Finalizes and shuts down the eCAL runtime system.
    ///
    /// After calling this function, all publishers, subscribers, and services are invalidated.
    pub fn finalize() {
        unsafe {
            rustecal_sys::eCAL_Finalize();
        }
    }

    /// Checks if the eCAL system is initialized and running properly.
    ///
    /// This can be used as the main loop condition in long-running processes.
    ///
    /// # Returns
    ///
    /// `true` if the system is operational, `false` otherwise.
    pub fn ok() -> bool {
        unsafe { rustecal_sys::eCAL_Ok() != 0 }
    }

    /// Checks if the eCAL system has been initialized.
    ///
    /// This function checks whether any components of the middleware have been initialized.
    ///
    /// # Returns
    ///
    /// `true` if initialization has occurred, `false` otherwise.
    pub fn is_initialized() -> bool {
        unsafe { rustecal_sys::eCAL_IsInitialized() != 0 }
    }

    /// Checks if specific components of eCAL are initialized.
    ///
    /// This allows querying the status of individual middleware components like pub/sub, monitoring, etc.
    ///
    /// # Arguments
    ///
    /// * `components` - Bitmask of components to check.
    ///
    /// # Returns
    ///
    /// `true` if the given components are initialized, `false` otherwise.
    pub fn is_component_initialized(components: EcalComponents) -> bool {
        unsafe { rustecal_sys::eCAL_IsComponentInitialized(components.bits()) != 0 }
    }

    /// Returns the version string of the eCAL runtime.
    ///
    /// # Returns
    ///
    /// A static string slice with the full version string, e.g. `"5.11.0"`.
    pub fn version_string() -> &'static str {
        unsafe {
            CStr::from_ptr(rustecal_sys::eCAL_GetVersionString())
                .to_str()
                .unwrap_or("unknown")
        }
    }

    /// Returns the build date string of the eCAL runtime.
    ///
    /// # Returns
    ///
    /// A static string slice with the build date string, e.g. `"Apr 2025"`.
    pub fn version_date_string() -> &'static str {
        unsafe {
            CStr::from_ptr(rustecal_sys::eCAL_GetVersionDateString())
                .to_str()
                .unwrap_or("unknown")
        }
    }

    /// Returns the version of the eCAL runtime as structured integers.
    ///
    /// # Returns
    ///
    /// A [`Version`] struct with fields `major`, `minor`, and `patch`.
    pub fn version_struct() -> Version {
        unsafe { rustecal_sys::eCAL_GetVersion().into() }
    }
}
