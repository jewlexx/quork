use winapi::{
    ctypes::c_void,
    shared::ntdef::NULL,
    um::{
        processthreadsapi::{GetCurrentProcess, OpenProcessToken},
        winnt::TOKEN_QUERY,
    },
};

extern "C" {
    fn IsElevated() -> u8;
}

/// Checks if the user is an administrator.
pub fn is_admin() -> bool {
    unsafe {
        let process = GetCurrentProcess();

        let mut htoken: *mut c_void = NULL;

        let token = OpenProcessToken(process, TOKEN_QUERY, &mut htoken);
    };

    unsafe { IsElevated() != 0 }
}
