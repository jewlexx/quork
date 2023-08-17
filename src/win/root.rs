//! Checks if process has root privelages

use std::{
    ffi::c_void,
    mem::{self, MaybeUninit},
};

use windows::Win32::{
    Foundation::{CloseHandle, INVALID_HANDLE_VALUE},
    Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};

/// Checks if the process is elevated
pub fn is_elevated() -> windows::core::Result<bool> {
    unsafe {
        let mut token = INVALID_HANDLE_VALUE;

        // TODO: Handle errors better
        let elevated = {
            OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token)?;

            let mut elevation: MaybeUninit<TOKEN_ELEVATION> = MaybeUninit::uninit();
            let mut size = mem::size_of::<TOKEN_ELEVATION>() as u32;

            GetTokenInformation(
                token,
                TokenElevation,
                Some(elevation.as_mut_ptr() as *mut c_void),
                size,
                &mut size,
            )?;

            elevation.assume_init().TokenIsElevated != 0
        };

        if token != INVALID_HANDLE_VALUE {
            CloseHandle(token)?;
        }

        Ok(elevated)
    }
}
