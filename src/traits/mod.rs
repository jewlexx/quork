//! Trait implementations

pub mod flip;
pub mod map;
pub mod truncate;
cfg_if::cfg_if! {
    // TODO: Get these working without std crate (as much as possible)
    if #[cfg(feature = "std")] {
    }
}

pub mod list;
pub mod truthy;

/// Trait prelude imports
pub mod prelude {
    cfg_if::cfg_if! {
        // TODO: Get these working without std crate (as much as possible)
        if #[cfg(feature = "std")] {
            pub use super::flip::*;
            pub use super::truncate::*;
            pub use super::map::*;
        }
    }

    pub use super::list::*;
    pub use super::truthy::*;
}
