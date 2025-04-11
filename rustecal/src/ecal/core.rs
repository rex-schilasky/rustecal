//! Core lifecycle control for the eCAL runtime.
//!
//! This module provides safe Rust wrappers around initializing and finalizing
//! the eCAL communication system, as well as querying its state.
//!
//! The main entry point is the [`Ecal`] struct which provides:
//! - [`Ecal::initialize`] to start the middleware
//! - [`Ecal::finalize`] to shut it down
//! - [`Ecal::ok`] to query if eCAL is currently running
//!
//! Typically, you will call [`Ecal::initialize`] once at the beginning of your
//! application and [`Ecal::finalize`] at shutdown.

use std::ffi::CString;
use crate::ecal::components::EcalComponents;
use rustecal_sys;

/// Provides access to the core initialization and finalization functions of eCAL.
pub struct Ecal;

impl Ecal {
    /// Initializes the eCAL runtime system.
    ///
    /// This function must be called before using any publisher or subscriber functionality.
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
    /// After calling this function, all publishers and subscribers are invalidated.
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
}
