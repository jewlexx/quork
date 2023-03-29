use std::{ffi::c_void, mem};

use windows::Win32::{
    Foundation::{CloseHandle, INVALID_HANDLE_VALUE, TRUE},
    Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};

pub fn is_elevated() -> bool {
    let mut token = INVALID_HANDLE_VALUE;
    unsafe {
        let elevated = if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == TRUE {
            let mut elevation: TOKEN_ELEVATION = mem::zeroed();
            let mut size = mem::size_of::<TOKEN_ELEVATION>() as u32;
            if GetTokenInformation(
                token,
                TokenElevation,
                Some(&mut elevation as *mut TOKEN_ELEVATION as *mut c_void),
                size,
                &mut size,
            ) == TRUE
            {
                elevation.TokenIsElevated != 0
            } else {
                false
            }
        } else {
            false
        };

        if token != INVALID_HANDLE_VALUE {
            CloseHandle(token);
        }

        elevated
    }
}
