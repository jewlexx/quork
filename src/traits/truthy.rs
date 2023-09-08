//! A simple trait to check if a value contains truth

/// A simple trait to check if a value contains truth
pub trait ContainsTruth {
    /// Check if the value contains truth
    fn contains_truth(&self) -> bool;
}

impl ContainsTruth for Option<bool> {
    fn contains_truth(&self) -> bool {
        matches!(self, Some(true))
    }
}

impl<E> ContainsTruth for Result<bool, E> {
    fn contains_truth(&self) -> bool {
        matches!(self, Ok(true))
    }
}
