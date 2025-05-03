//! Trait implementations

pub mod flip;
pub mod list;
pub mod map;
pub mod truncate;
pub mod truthy;

/// Trait prelude imports
pub mod prelude {
    pub use super::flip::*;
    pub use super::list::*;
    pub use super::map::*;
    pub use super::truncate::*;
    pub use super::truthy::*;
}
