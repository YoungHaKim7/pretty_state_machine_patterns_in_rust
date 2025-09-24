// Bottle filling machine definition
// This is the main state machine that holds the current state

use crate::states::{Waiting, Filling, Done};

/// Generic state machine for bottle filling operations
/// The type parameter S represents the current state
#[derive(Debug)]
pub struct BottleFillingMachine<S> {
    /// Value shared across all states
    pub shared_value: usize,
    /// Current state of the machine
    pub state: S,
}

/// Implementation for the machine when it's in the Waiting state
impl BottleFillingMachine<Waiting> {
    /// Creates a new bottle filling machine in the Waiting state
    pub fn new(shared_value: usize) -> Self {
        BottleFillingMachine {
            shared_value,
            state: Waiting::with_shared_value(shared_value),
        }
    }

    /// Gets the waiting time from the current state
    pub fn waiting_time(&self) -> std::time::Duration {
        self.state.waiting_time
    }
}

/// Implementation for the machine when it's in the Filling state
impl BottleFillingMachine<Filling> {
    /// Gets the filling rate from the current state
    pub fn filling_rate(&self) -> usize {
        self.state.rate
    }
}

/// Implementation for the machine when it's in the Done state
impl BottleFillingMachine<Done> {
    /// Checks if the machine is done
    pub fn is_done(&self) -> bool {
        true
    }
}

/// Generic implementation for any state
impl<S> BottleFillingMachine<S> {
    /// Gets the shared value from the machine
    pub fn shared_value(&self) -> usize {
        self.shared_value
    }

    /// Updates the shared value
    pub fn set_shared_value(&mut self, value: usize) {
        self.shared_value = value;
    }
}