//! Checks if process has root privelages

use core::mem::{self, MaybeUninit};

use windows_sys::Win32::{
    Foundation::{CloseHandle, INVALID_HANDLE_VALUE},
    Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};

use super::{Error, Result};

/// Checks if the process is elevated
///
/// # Errors
/// - Cannot open process token ([`OpenProcessToken`])
/// - Cannot get token information ([`GetTokenInformation`])
/// - Cannot close handle ([`CloseHandle`])
pub fn is_elevated() -> Result<bool> {
    unsafe {
        let mut token = INVALID_HANDLE_VALUE;

        // TODO: Handle errors better
        let elevated = {
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
                return Err(Error::ProcessToken);
            }

            let mut elevation: MaybeUninit<TOKEN_ELEVATION> = MaybeUninit::uninit();
            #[allow(clippy::cast_possible_truncation)]
            let mut size = mem::size_of::<TOKEN_ELEVATION>() as u32;

            if GetTokenInformation(
                token,
                TokenElevation,
                elevation.as_mut_ptr().cast(),
                size,
                &mut size,
            ) == 0
            {
                if CloseHandle(token) == 0 {
                    return Err(Error::CloseHandle);
                }
                return Err(Error::TokenInformation);
            }

            elevation.assume_init().TokenIsElevated != 0
        };

        if token != INVALID_HANDLE_VALUE && CloseHandle(token) == 0 {
            return Err(Error::CloseHandle);
        }

        Ok(elevated)
    }
}
