extern "C" {
    fn IsElevated() -> u8;
}

pub fn is_admin() -> bool {
    unsafe { IsElevated() != 0 }
}
