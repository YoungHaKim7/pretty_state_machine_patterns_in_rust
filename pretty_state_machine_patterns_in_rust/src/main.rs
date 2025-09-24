#[derive(Debug)]
struct Machine;

#[derive(Debug)]
struct BottleFiller {
    state: BottleFillerState
}

#[derive(Debug)]
enum BottleFillerState {
    Waiting { waiting_time: std::time::Duration },
    Filling { rate: usize },
    Done
}

#[derive(Debug)]
struct StateMachine {
    state: BottleFillerState
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            state: BottleFillerState::Waiting { 
                waiting_time: std::time::Duration::new(0, 0) 
            }
        }
    }

    fn to_filling(&mut self) {
        self.state = match &self.state {
            BottleFillerState::Waiting { .. } => BottleFillerState::Filling { rate: 1 },
            _ => panic!("Invalid state transition!"),
        }
    }
}


fn main() {
    let my_machine = Machine;
    let mut state_machine = StateMachine::new();

    println!("my_machine = {my_machine:?}");
    println!("Initial state: {:?}", state_machine.state);
    
    state_machine.to_filling();
    println!("After transition: {:?}", state_machine.state);
}
