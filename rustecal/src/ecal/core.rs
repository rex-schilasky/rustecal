use std::ffi::CString;
use crate::ecal::components::EcalComponents;
use rustecal_sys;

pub struct Ecal;

impl Ecal {
    /// Initialize eCAL with specific components
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

    /// Finalize eCAL
    pub fn finalize() {
        unsafe {
            rustecal_sys::eCAL_Finalize();
        }
    }

    /// Check if eCAL is running and in a valid state
    pub fn ok() -> bool {
        unsafe { rustecal_sys::eCAL_Ok() != 0 }
    }
}
