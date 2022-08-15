//! Utilities for checking the users privelages

crate::import_ffi!();

/// Checks if the user is an administrator.
pub fn is_admin() -> bool {
    ffi::admin::is_admin()
}
