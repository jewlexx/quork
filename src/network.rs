//! Network helpers

/// A list of the possible ip versions
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IpVersion {
    /// IPv4
    V4,
    /// IPv6
    V6,
}

#[cfg(windows)]
pub use crate::win::network::*;
