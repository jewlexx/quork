//! Checks if the current user has root privileges.

cfg_if::cfg_if! {
    if #[cfg(windows)] {
        pub use crate::win::root::is_elevated as is_root;
    } else if #[cfg(unix)] {
        pub use crate::unix::root::is_root;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_root() {
        // You probably shouldn't be running cargo commands as root
        // Just checks if the command runs, and is consistent across the inspiration for the implementation
        assert_eq!(is_root(), is_root::is_root());
    }
}
