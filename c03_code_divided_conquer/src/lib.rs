// Module declarations
pub mod traits;
pub mod states;
pub mod machine;
pub mod transitions;

// Re-export commonly used types for convenience
pub use traits::*;
pub use states::*;
pub use machine::*;