use std::time::Duration;

// Shared functionality across all states
trait SharedFunctionality {
    fn get_shared_value(&self) -> usize;
}

// === State structs ===
#[derive(Debug)]
struct Waiting {
    waiting_time: Duration,
    shared_value: usize,
}
impl SharedFunctionality for Waiting {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

#[derive(Debug)]
struct Filling {
    rate: usize,
    shared_value: usize,
}
impl SharedFunctionality for Filling {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

#[derive(Debug)]
struct Done {
    bottles_filled: usize,
    shared_value: usize,
}
impl SharedFunctionality for Done {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

#[derive(Debug)]
struct Cleaning {
    cleaning_cycles: usize,
    shared_value: usize,
}
impl SharedFunctionality for Cleaning {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

#[derive(Debug)]
struct Maintenance {
    maintenance_hours: f64,
    shared_value: usize,
}
impl SharedFunctionality for Maintenance {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

#[derive(Debug)]
struct Error {
    error_code: String,
    shared_value: usize,
}
impl SharedFunctionality for Error {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

// === Enum wrapper ===
#[derive(Debug)]
enum State {
    Waiting(Waiting),
    Filling(Filling),
    Done(Done),
    Cleaning(Cleaning),
    Maintenance(Maintenance),
    Error(Error),
}

// === State machine ===
#[derive(Debug)]
struct BottleFillingMachine {
    state: State,
}

impl BottleFillingMachine {
    fn new(shared_value: usize) -> Self {
        println!("Creating BottleFillingMachine in Waiting state...");
        Self {
            state: State::Waiting(Waiting {
                waiting_time: Duration::new(0, 0),
                shared_value,
            }),
        }
    }

    fn to_filling(self) -> Self {
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

    fn to_done(self) -> Self {
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

    fn reset_waiting(self) -> Self {
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

    fn to_cleaning(self) -> Self {
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

    fn to_maintenance(self) -> Self {
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

    fn to_error(self, error_code: String) -> Self {
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

    fn recover_from_error(self) -> Self {
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

// === MAIN ===
fn main() {
    let mut machine = BottleFillingMachine::new(0);
    println!("Debug #1: {:?}", machine);

    // Access shared value if in Waiting
    if let State::Waiting(w) = &machine.state {
        println!("Debug #2: Waiting time = {:?}", w.waiting_time);
        println!("Debug #3: Shared value = {}", w.get_shared_value());
    }

    // Transition to Filling
    machine = machine.to_filling();
    println!("Debug #4: {:?}", machine);

    if let State::Filling(f) = &machine.state {
        println!("Debug #5: Filling rate = {}", f.rate);
        println!("Debug #6: Shared value = {}", f.get_shared_value());
        println!("Debug #7: Is rate > 0 ? {}", f.rate > 0);
    }

    // Transition to Done
    machine = machine.to_done();
    println!("Debug #8: {:?}", machine);

    if let State::Done(d) = &machine.state {
        println!("Debug #9: Bottles filled = {}", d.bottles_filled);
        println!("Debug #10: Shared value = {}", d.get_shared_value());
        println!("Debug #11: Is final count > 0 ? {}", d.bottles_filled > 0);
    }

    // Reset back to Waiting
    machine = machine.reset_waiting();
    println!("Debug #12: {:?}", machine);

    if let State::Waiting(w) = &machine.state {
        println!("Debug #13: Waiting time reset = {:?}", w.waiting_time);
        println!(
            "Debug #14: Shared value after reset = {}",
            w.get_shared_value()
        );
    }

    // Extra debug prints to exceed 20
    println!(
        "Debug #15: Machine discriminant = {:?}",
        std::mem::discriminant(&machine.state)
    );
    println!(
        "Debug #16: Type check = {}",
        matches!(machine.state, State::Waiting(_))
    );
    println!(
        "Debug #17: Is Filling? {}",
        matches!(machine.state, State::Filling(_))
    );
    println!(
        "Debug #18: Is Done? {}",
        matches!(machine.state, State::Done(_))
    );
    println!("Debug #19: Full debug dump = {:?}", machine);
    println!(
        "Debug #20: Shared value (via match) = {}",
        match &machine.state {
            State::Waiting(w) => w.get_shared_value(),
            State::Filling(f) => f.get_shared_value(),
            State::Done(d) => d.get_shared_value(),
            State::Cleaning(c) => c.get_shared_value(),
            State::Maintenance(m) => m.get_shared_value(),
            State::Error(e) => e.get_shared_value(),
        }
    );

    // === NEW STATES DEMONSTRATION ===
    println!("\n=== NEW STATES DEMONSTRATION ===");
    
    // Go through Done → Cleaning → Maintenance flow
    machine = machine.to_done();
    println!("Debug #22: {:?}", machine);
    
    machine = machine.to_cleaning();
    println!("Debug #23: {:?}", machine);
    
    if let State::Cleaning(c) = &machine.state {
        println!("Debug #24: Cleaning cycles = {}", c.cleaning_cycles);
        println!("Debug #25: Shared value = {}", c.get_shared_value());
    }
    
    machine = machine.to_maintenance();
    println!("Debug #26: {:?}", machine);
    
    if let State::Maintenance(m) = &machine.state {
        println!("Debug #27: Maintenance hours = {}", m.maintenance_hours);
        println!("Debug #28: Shared value = {}", m.get_shared_value());
    }
    
    // Demonstrate error handling
    machine = machine.to_error("VALVE_STUCK".to_string());
    println!("Debug #29: {:?}", machine);
    
    if let State::Error(e) = &machine.state {
        println!("Debug #30: Error code = {}", e.error_code);
        println!("Debug #31: Shared value = {}", e.get_shared_value());
    }
    
    // Recover from error
    machine = machine.recover_from_error();
    println!("Debug #32: {:?}", machine);
    
    if let State::Waiting(w) = &machine.state {
        println!("Debug #33: Recovered to Waiting, shared value = {}", w.get_shared_value());
    }
    
    // Test error from different state
    machine = machine.to_filling();
    machine = machine.to_error("PUMP_FAILURE".to_string());
    println!("Debug #34: {:?}", machine);
    
    if let State::Error(e) = &machine.state {
        println!("Debug #35: Error from Filling state: {}", e.error_code);
    }
    
    println!("Debug #36: Program finished with new states!");
}
