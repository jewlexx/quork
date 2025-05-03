//! Implement truncation for types

use crate::truncate::Truncate;

/// Truncate the data when formatting
pub trait Truncation
where
    Self: Sized + core::fmt::Display,
{
    /// Truncate the data
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quork::truncate::Truncation;
    /// let name = "Juliette Cordor".truncate(8);
    ///
    /// assert_eq!(name.to_string(), "Juliette");
    /// ```
    fn truncate(self, length: usize) -> Truncate<Self> {
        Truncate::new(self, length)
    }
}

impl<T: core::fmt::Display> Truncation for T {}
