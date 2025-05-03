//! Windows specific functionality.

use windows_sys::Win32::Foundation::GetLastError;

#[cfg(feature = "network")]
pub mod network;
#[cfg(feature = "root")]
pub mod root;

mod com;

/// A type alias for the result type used in this crate.
/// It is a `Result` type with the error type set to `Error` by default.
pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Copy, Clone)]
/// Errors that are specific to the windows platform
pub enum Error {
    /// Failed to open process token.
    ProcessToken,
    /// Failed to get token information.
    TokenInformation,
    /// Failed to close handle.
    CloseHandle,
}

impl Error {
    const fn message(self) -> &'static str {
        match self {
            Error::ProcessToken => "Failed to open process token.",
            Error::TokenInformation => "Failed to get token information.",
            Error::CloseHandle => "Failed to close handle.",
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let win_error = unsafe { GetLastError() };

        f.write_str(self.message())?;

        write!(f, "Windows error code: {win_error:x}.")
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &'static str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn core::error::Error> {
        self.source()
    }
}
