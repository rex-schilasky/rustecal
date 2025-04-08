pub struct Ecal;

impl Ecal {
    pub fn initialize(unit_name: Option<&str>) -> Result<(), i32> {
        let cstr = unit_name
            .map(|s| std::ffi::CString::new(s).unwrap());
        let ptr = cstr.as_ref().map_or(std::ptr::null(), |s| s.as_ptr());

        let result = unsafe {
            rustecal_sys::eCAL_Initialize(ptr, std::ptr::null(), std::ptr::null())
        };

        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    pub fn finalize() {
        unsafe {
            rustecal_sys::eCAL_Finalize();
        }
    }
}
