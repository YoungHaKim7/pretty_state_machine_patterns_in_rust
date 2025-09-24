// State definitions for the bottle filling machine
// Each state represents a different phase in the bottle filling process

use crate::traits::SharedFunctionality;

/// Waiting state - represents the machine waiting for a bottle
#[derive(Debug)]
pub struct Waiting {
    pub waiting_time: std::time::Duration,
    /// Value shared by all states
    pub shared_value: usize,
}

impl Waiting {
    /// Creates a new Waiting state with default values
    pub fn new() -> Self {
        Waiting {
            waiting_time: std::time::Duration::new(0, 0),
            shared_value: 0,
        }
    }

    /// Creates a new Waiting state with a specific shared value
    pub fn with_shared_value(shared_value: usize) -> Self {
        Waiting {
            waiting_time: std::time::Duration::new(0, 0),
            shared_value,
        }
    }

    /// Transitions from Waiting to Filling state (consumes self)
    pub fn to_filling(self) -> Filling {
        Filling {
            rate: 1,
            shared_value: self.shared_value,
        }
    }
}

impl SharedFunctionality for Waiting {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

/// Filling state - represents the machine actively filling a bottle
#[derive(Debug)]
pub struct Filling {
    pub rate: usize,
    /// Value shared by all states
    pub shared_value: usize,
}

impl Filling {
    /// Creates a new Filling state with specified rate and shared value
    pub fn new(rate: usize, shared_value: usize) -> Self {
        Filling { rate, shared_value }
    }
}

impl SharedFunctionality for Filling {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

/// Done state - represents the machine having completed filling
#[derive(Debug)]
pub struct Done;

impl SharedFunctionality for Done {
    fn get_shared_value(&self) -> usize {
        // Done state doesn't have a shared_value field, so we return 0
        // In a real implementation, you might want to store this differently
        0
    }
}