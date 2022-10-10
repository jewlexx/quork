#![warn(missing_docs)]

//! A little crate that helps with interacting with the system
//!
//! The lower levels can be hard. So this crate is intended to help with that by abstracing the lower levels to high level code

#[cfg(feature = "macros")]
pub use quork_proc as macros;

#[cfg(feature = "flip")]
pub mod flip;

pub mod prelude;

mod imacros;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(not(target_os = "windows"))]
mod unix;

#[cfg(feature = "admin")]
pub mod admin;

/// A simple trait to map the return value of a Mutex in a nice little closure
pub trait LockMap<T: ?Sized> {
    /// Maps the return value of a Mutex in a closure,
    /// dropping and unlocking the Mutex after execution and returning the result
    fn map<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        f(self)
    }
}

impl<'a, T> LockMap<T> for std::sync::MutexGuard<'a, T> {}

#[cfg(feature = "spin")]
impl<'a, T> LockMap<T> for spin::mutex::MutexGuard<'a, T> {}

#[cfg(feature = "parking_lot")]
impl<'a, T> LockMap<T> for parking_lot::MutexGuard<'a, T> {}
