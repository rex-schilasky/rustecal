use thiserror::Error;

/// All error types returned by rustecal‑core.
#[derive(Debug, Error)]
pub enum RustecalError {
    /// A non‑zero return code from the eCAL C API.
    #[error("eCAL error code {0}")]
    Ecal(i32),

    /// Unexpected null pointer from C.
    #[error("unexpected null pointer")]
    NullPointer,

    /// A catch‑all for any other internal Rust error.
    #[error("internal error: {0}")]
    Internal(String),
}

/// Check a C return code: `0` → `Ok(())`, non‑zero → `Err(RustecalError::Ecal)`.
pub fn check(code: i32) -> Result<(), RustecalError> {
    if code == 0 {
        Ok(())
    } else {
        Err(RustecalError::Ecal(code))
    }
}
