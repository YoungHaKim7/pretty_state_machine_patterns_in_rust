use std::time::Duration;
use crate::states::*;

// === State machine ===
#[derive(Debug)]
pub struct BottleFillingMachine {
    pub state: State,
}

impl BottleFillingMachine {
    pub fn new(shared_value: usize) -> Self {
        println!("Creating BottleFillingMachine in Waiting state...");
        Self {
            state: State::Waiting(Waiting {
                waiting_time: Duration::new(0, 0),
                shared_value,
            }),
        }
    }
}