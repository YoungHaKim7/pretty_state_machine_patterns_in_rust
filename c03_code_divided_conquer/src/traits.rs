// Shared functionality across all states
pub trait SharedFunctionality {
    fn get_shared_value(&self) -> usize;
}