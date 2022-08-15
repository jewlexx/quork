//! Utilities for checking the users privelages

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        use crate::windows as ffi;
    } else {
        use crate::unix as ffi;
    }
}

/// Checks if the user is an administrator.
pub fn is_admin() -> bool {
    ffi::admin::is_admin()
}
