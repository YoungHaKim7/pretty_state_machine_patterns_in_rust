// State transition implementations
// This module defines how the machine transitions between different states

use crate::machine::BottleFillingMachine;
use crate::states::{Waiting, Filling, Done};

/// Transition from Waiting state to Filling state
impl From<BottleFillingMachine<Waiting>> for BottleFillingMachine<Filling> {
    fn from(val: BottleFillingMachine<Waiting>) -> BottleFillingMachine<Filling> {
        BottleFillingMachine {
            shared_value: val.shared_value,
            state: Filling::new(1, val.shared_value),
        }
    }
}

/// Transition from Filling state to Done state
impl From<BottleFillingMachine<Filling>> for BottleFillingMachine<Done> {
    fn from(val: BottleFillingMachine<Filling>) -> BottleFillingMachine<Done> {
        BottleFillingMachine {
            shared_value: val.shared_value,
            state: Done,
        }
    }
}

/// Transition from Done state back to Waiting state (cycle completion)
impl From<BottleFillingMachine<Done>> for BottleFillingMachine<Waiting> {
    fn from(val: BottleFillingMachine<Done>) -> BottleFillingMachine<Waiting> {
        BottleFillingMachine {
            shared_value: val.shared_value,
            state: Waiting::with_shared_value(val.shared_value),
        }
    }
}

/// Additional transition methods for convenience
impl BottleFillingMachine<Waiting> {
    /// Transitions to Filling state
    pub fn start_filling(self) -> BottleFillingMachine<Filling> {
        self.into()
    }
}

impl BottleFillingMachine<Filling> {
    /// Transitions to Done state
    pub fn finish_filling(self) -> BottleFillingMachine<Done> {
        self.into()
    }
}

impl BottleFillingMachine<Done> {
    /// Transitions back to Waiting state
    pub fn reset_to_waiting(self) -> BottleFillingMachine<Waiting> {
        self.into()
    }
}