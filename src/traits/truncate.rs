//! Implement truncation for types

use crate::truncate::Truncate as TruncateStruct;

/// Truncate the data when formatting
pub trait Truncate
where
    Self: Sized + std::fmt::Display,
{
    /// Truncate the data
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quork::traits::truncate::Truncate;
    /// let name = "Juliette Cordor".truncate(8);
    ///
    /// assert_eq!(name.to_string(), "Juliette");
    /// ```
    fn truncate(self, length: usize) -> TruncateStruct<Self> {
        TruncateStruct::new(self, length)
    }
}

impl<T: std::fmt::Display> Truncate for T {}
