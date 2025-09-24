// State machine library with modular design
// This demonstrates a "divide and conquer" approach to organizing Rust code

pub mod traits;
pub mod states;
pub mod machine;
pub mod transitions;

// Re-export commonly used types for convenience
pub use traits::SharedFunctionality;
pub use states::{Waiting, Filling, Done};
pub use machine::BottleFillingMachine;