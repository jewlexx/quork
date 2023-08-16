//! Checks user root privelages

use std::{
    ffi::c_void,
    mem::{self, MaybeUninit},
};

use windows::Win32::{
    Foundation::{CloseHandle, INVALID_HANDLE_VALUE},
    Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};

/// Checks if a user is elevated
pub fn is_elevated() -> bool {
    unsafe {
        let mut token = INVALID_HANDLE_VALUE;

        // TODO: Handle errors better
        let elevated = if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_ok() {
            let mut elevation: MaybeUninit<TOKEN_ELEVATION> = MaybeUninit::uninit();
            let mut size = mem::size_of::<TOKEN_ELEVATION>() as u32;
            if GetTokenInformation(
                token,
                TokenElevation,
                Some(elevation.as_mut_ptr() as *mut c_void),
                size,
                &mut size,
            )
            .is_ok()
            {
                elevation.assume_init().TokenIsElevated != 0
            } else {
                false
            }
        } else {
            false
        };

        if token != INVALID_HANDLE_VALUE {
            _ = CloseHandle(token);
        }

        elevated
    }
}
