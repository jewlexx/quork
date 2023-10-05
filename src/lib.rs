#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod prelude {
    //! `use quork::prelude::*` To include common helpful items

    cfg_if::cfg_if! {
        if #[cfg(feature = "traits")] {
            pub use crate::traits::flip::*;
            pub use crate::traits::lock::*;
            pub use crate::traits::truthy::*;
            pub use crate::traits::list::*;
        }
    }

    #[cfg(feature = "macros")]
    pub use crate::macros::*;

    #[cfg(feature = "root")]
    pub use crate::root::is_root;
}

#[cfg(windows)]
pub mod win;

#[cfg(unix)]
pub mod unix;

#[cfg(feature = "macros")]
pub mod macros;

#[cfg(feature = "traits")]
pub mod traits;

#[cfg(feature = "network")]
pub mod network;

cfg_if::cfg_if! {
    if #[cfg(feature = "root")] {
        pub mod root;
    }
}

/// Truncation helpers for truncating strings when formatting
pub mod truncate {
    use std::fmt;

    pub use crate::traits::truncate::Truncation;

    #[derive(Debug)]
    #[must_use]
    /// A wrapper that truncates its contents when used.
    ///
    /// This should be used through the [`fmt::Display`] trait,
    /// in [`println!`] and other formatting circumstances
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use quork::truncate::Truncate;
    /// let name = Truncate::new("Juliette Cordor", 8);
    ///
    /// assert_eq!(name.to_string(), "Juliette")
    /// ```
    pub struct Truncate<T> {
        length: usize,
        data: T,
        suffix: Option<String>,
    }

    impl<T> Truncate<T> {
        /// Construct a new [`Truncate`] from the provided data and length
        pub fn new(data: T, length: usize) -> Self {
            Self {
                length,
                data,
                suffix: None,
            }
        }

        /// Add a suffix to the provided [`Truncate`], to print after the truncation
        ///
        /// # Examples
        ///
        /// ```rust
        /// # use quork::truncate::Truncate;
        /// let mut name = Truncate::new("Juliette Cordor", 8).with_suffix("...");
        ///
        /// assert_eq!(name.to_string(), "Juliette...")
        /// ```
        pub fn with_suffix(self, suffix: impl fmt::Display) -> Self {
            Self {
                suffix: Some(suffix.to_string()),
                ..self
            }
        }
    }

    impl<T: fmt::Display> fmt::Display for Truncate<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use fmt::Write;
            let mut truncator = TruncatedFormatter::new(f, self.length);

            write!(truncator, "{}", self.data)?;

            if let Some(ref suffix) = self.suffix {
                write!(f, "{suffix}")?;
            }

            Ok(())
        }
    }

    #[derive(Debug)]
    #[must_use]
    /// A wrapper over a writer that truncates the output to a certain length
    ///
    /// Primarily intended to be used through the [`Truncate`] struct
    pub struct TruncatedFormatter<'a, T> {
        remaining: usize,
        writer: &'a mut T,
    }

    impl<'a, T> TruncatedFormatter<'a, T> {
        /// Construct a new [`TruncatedFormatter`] from the provided writer and length
        pub fn new(writer: &'a mut T, length: usize) -> Self {
            Self {
                remaining: length,
                writer,
            }
        }
    }

    impl<'a, T> fmt::Write for TruncatedFormatter<'a, T>
    where
        T: fmt::Write,
    {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.remaining < s.len() {
                self.writer.write_str(&s[0..self.remaining])?;
                self.remaining = 0;
                Ok(())
            } else {
                self.remaining -= s.len();
                self.writer.write_str(s)
            }
        }
    }
}
