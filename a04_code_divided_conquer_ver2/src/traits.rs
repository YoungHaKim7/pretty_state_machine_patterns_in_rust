// Shared functionality trait for all states
// This trait defines common behavior that all states must implement

/// Trait that defines shared functionality across all states
pub trait SharedFunctionality {
    /// Returns the shared value associated with this state
    fn get_shared_value(&self) -> usize;
}