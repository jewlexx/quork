extern "C" {
    fn IsElevated() -> u8;
}

/// Checks if the user is an administrator.
pub fn is_admin() -> bool {
    unsafe { IsElevated() != 0 }
}
