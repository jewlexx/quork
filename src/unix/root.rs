//! Checks if process has root privelages

/// Checks if the process is running as root
pub fn is_root() -> bool {
    nix::unistd::Uid::effective().is_root()
}
