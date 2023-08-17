//! Listing all Variants of an enum

#[cfg(feature = "macros")]
pub use crate::macros::ListVariants;

/// A trait listing all Variants of an enum
///
/// This can technically be implemented for nearly anything, but it is designed for enums, in conjunction with the associated derive macro
pub trait ListVariants<const N: usize>
where
    Self: Sized,
{
    /// All variants of the enum
    const VARIANTS: [Self; N];
}
