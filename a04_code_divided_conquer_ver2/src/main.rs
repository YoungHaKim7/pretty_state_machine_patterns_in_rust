// Main entry point demonstrating the refactored state machine
// This shows how the "divide and conquer" approach makes the code more organized

use a04_code_divided_conquer_ver2::{
    BottleFillingMachine, Waiting, Filling, Done, SharedFunctionality
};

fn main() {
    println!("=== Bottle Filling Machine Demo ===");
    
    // Create a new machine in Waiting state
    let bottle_filler = BottleFillingMachine::new(42);
    
    println!("Initial state:");
    println!("  Machine: {:?}", bottle_filler);
    println!("  Shared value: {}", bottle_filler.shared_value());
    println!("  Waiting time: {:?}", bottle_filler.waiting_time());
    println!("  State shared value: {}", bottle_filler.state.get_shared_value());
    
    // Verify initial state
    assert_eq!(bottle_filler.state.waiting_time, std::time::Duration::new(0, 0));
    assert_eq!(bottle_filler.shared_value(), 42);
    
    println!("\n--- Transitioning to Filling state ---");
    
    // Transition to Filling state using the From trait
    let bottle_filler = BottleFillingMachine::<Filling>::from(bottle_filler);
    
    println!("Filling state:");
    println!("  Machine: {:?}", bottle_filler);
    println!("  Shared value: {}", bottle_filler.shared_value());
    println!("  Filling rate: {}", bottle_filler.filling_rate());
    println!("  State shared value: {}", bottle_filler.state.get_shared_value());
    
    // Note: Can't access waiting_time anymore - the state has changed!
    // This demonstrates the type safety of the state machine
    
    println!("\n--- Transitioning to Done state ---");
    
    // Transition to Done state
    let bottle_filler = BottleFillingMachine::<Done>::from(bottle_filler);
    
    println!("Done state:");
    println!("  Machine: {:?}", bottle_filler);
    println!("  Shared value: {}", bottle_filler.shared_value());
    println!("  Is done: {}", bottle_filler.is_done());
    println!("  State shared value: {}", bottle_filler.state.get_shared_value());
    
    println!("\n--- Transitioning back to Waiting state ---");
    
    // Transition back to Waiting state (completing the cycle)
    let bottle_filler = BottleFillingMachine::<Waiting>::from(bottle_filler);
    
    println!("Back to Waiting state:");
    println!("  Machine: {:?}", bottle_filler);
    println!("  Shared value: {}", bottle_filler.shared_value());
    println!("  Waiting time: {:?}", bottle_filler.waiting_time());
    println!("  State shared value: {}", bottle_filler.state.get_shared_value());
    
    println!("\n--- Demonstrating state-specific methods ---");
    
    // Demonstrate state-specific functionality
    let waiting_state = Waiting::new();
    println!("Standalone Waiting state: {:?}", waiting_state);
    
    let filling_state = waiting_state.to_filling();
    println!("Converted to Filling state: {:?}", filling_state);
    
    println!("\n=== Demo Complete ===");
    println!("The state machine successfully demonstrates:");
    println!("  ✓ Type-safe state transitions");
    println!("  ✓ Shared state preservation");
    println!("  ✓ Modular, organized code structure");
    println!("  ✓ Clear separation of concerns");
}