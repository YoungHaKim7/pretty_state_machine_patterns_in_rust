use std::time::Duration;
use crate::machine::BottleFillingMachine;
use crate::states::*;
use crate::traits::SharedFunctionality;

impl BottleFillingMachine {
    pub fn to_filling(self) -> Self {
        println!("Transitioning: Waiting → Filling");
        match self.state {
            State::Waiting(w) => BottleFillingMachine {
                state: State::Filling(Filling {
                    rate: 1,
                    shared_value: w.shared_value + 1,
                }),
            },
            _ => {
                println!("Invalid transition to Filling");
                self
            }
        }
    }

    pub fn to_done(self) -> Self {
        println!("Transitioning: Filling → Done");
        match self.state {
            State::Filling(f) => BottleFillingMachine {
                state: State::Done(Done {
                    bottles_filled: 42, // mock result
                    shared_value: f.shared_value + 10,
                }),
            },
            _ => {
                println!("Invalid transition to Done");
                self
            }
        }
    }

    pub fn reset_waiting(self) -> Self {
        println!("Transitioning: Done → Waiting");
        match self.state {
            State::Done(d) => BottleFillingMachine {
                state: State::Waiting(Waiting {
                    waiting_time: Duration::new(0, 0),
                    shared_value: d.shared_value + 100,
                }),
            },
            _ => {
                println!("Invalid reset to Waiting");
                self
            }
        }
    }

    pub fn to_cleaning(self) -> Self {
        println!("Transitioning: Done → Cleaning");
        match self.state {
            State::Done(d) => BottleFillingMachine {
                state: State::Cleaning(Cleaning {
                    cleaning_cycles: 1,
                    shared_value: d.shared_value + 50,
                }),
            },
            _ => {
                println!("Invalid transition to Cleaning");
                self
            }
        }
    }

    pub fn to_maintenance(self) -> Self {
        println!("Transitioning: Cleaning → Maintenance");
        match self.state {
            State::Cleaning(c) => BottleFillingMachine {
                state: State::Maintenance(Maintenance {
                    maintenance_hours: 2.5,
                    shared_value: c.shared_value + 200,
                }),
            },
            _ => {
                println!("Invalid transition to Maintenance");
                self
            }
        }
    }

    pub fn to_error(self, error_code: String) -> Self {
        println!("Transitioning: Any → Error");
        let current_shared_value = match &self.state {
            State::Waiting(w) => w.get_shared_value(),
            State::Filling(f) => f.get_shared_value(),
            State::Done(d) => d.get_shared_value(),
            State::Cleaning(c) => c.get_shared_value(),
            State::Maintenance(m) => m.get_shared_value(),
            State::Error(e) => e.get_shared_value(),
        };

        BottleFillingMachine {
            state: State::Error(Error {
                error_code,
                shared_value: current_shared_value + 1000,
            }),
        }
    }

    pub fn recover_from_error(self) -> Self {
        println!("Transitioning: Error → Waiting");
        match self.state {
            State::Error(e) => BottleFillingMachine {
                state: State::Waiting(Waiting {
                    waiting_time: Duration::new(0, 0),
                    shared_value: e.shared_value + 500,
                }),
            },
            _ => {
                println!("Invalid recovery from Error");
                self
            }
        }
    }
}