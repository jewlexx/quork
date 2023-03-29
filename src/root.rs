//! Checks if the current user has root privileges.

cfg_if::cfg_if! {
    if #[cfg(windows)] {
        pub use crate::win::root::is_elevated as is_root;
    } else if #[cfg(unix)] {
        pub use crate::unix::root::is_root;
    }
}
