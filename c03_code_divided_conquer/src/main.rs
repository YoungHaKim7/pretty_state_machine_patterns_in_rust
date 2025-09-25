// Import the modular state machine components
use c03_code_divided_conquer::*;

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
        println!(
            "Debug #33: Recovered to Waiting, shared value = {}",
            w.get_shared_value()
        );
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
