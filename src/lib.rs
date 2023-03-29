#![warn(missing_docs)]

//! A little crate that helps with interacting with the system
//!
//! The lower levels can be hard. So this crate is intended to help with that by abstracing the lower levels to high level code

pub mod prelude {
    //! `use quork::prelude::*` To include common helpful items

    pub use crate::macros::*;
    pub use crate::LockMap;

    #[cfg(feature = "flip")]
    pub use crate::flip::*;
}

#[cfg(feature = "macros")]
pub use quork_proc as macros;

#[cfg(feature = "flip")]
pub mod flip;

#[cfg(windows)]
mod win;

#[cfg(all(windows, feature = "network"))]
pub use win::network;

#[cfg(feature = "root")]
pub use is_root::is_root;

/// Defines whether a struct is true
pub trait IsTrue {
    /// Defines whether a given type is true
    fn is_true(&self) -> bool;
}

impl IsTrue for Option<bool> {
    /// Checks whether the Option is Some and contains a true value
    fn is_true(&self) -> bool {
        matches!(self, Some(true))
    }
}

impl<T> IsTrue for Result<bool, T> {
    /// Checks whether the Result is Ok and contains a true value
    fn is_true(&self) -> bool {
        matches!(self, Ok(true))
    }
}

/// Map a mutex lock
pub trait LockMap<T: ?Sized> {
    /// Maps a mutex lock of T to a value of U
    fn map<F, U>(&mut self, f: F) -> U
    where
        F: FnOnce(&mut Self) -> U,
    {
        f(self)
    }
}

impl<'a, T> LockMap<T> for std::sync::MutexGuard<'a, T> {}

#[cfg(feature = "spin")]
impl<'a, T> LockMap<T> for spin::mutex::MutexGuard<'a, T> {}

#[cfg(feature = "parking_lot")]
impl<'a, T> LockMap<T> for parking_lot::MutexGuard<'a, T> {}
