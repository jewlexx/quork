//! Checks if process has root privileges

#[allow(clippy::module_name_repetitions)]
#[must_use]
/// Checks if the process is running as root
pub fn is_root() -> bool {
    nix::unistd::Uid::effective().is_root()
}
