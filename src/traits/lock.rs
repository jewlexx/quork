//! Map a Mutex Lock
//!
//! Can theoretically be anything but designed primarily for Mutex Locks

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
