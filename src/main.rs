#[derive(Debug)]
struct Machine;
struct BottleFiller {
    state : BottleFillerState
}

struct State {
    Waiting : { waiting_time: std::time::Duration}, 
    Filling :{rate: usize}, 
    Done
}

struct StateMachine {state: State}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            state: State::Waiting { waiting_time: std::time::Duration::new(0,0)}
        }
    }

    fn to_filling(&mut self) {
        self.state = match self.state {
            State::Waiting {} => State::Filling {rate:1},
            _ => panic!("Invalid state transition!"),
        }
    }
}

enum BottleFillerState {
    Waiting = {wating_time: std::time::Duration},
    Filling{rate:usize},
    Done
}


fn main() {
    let my_machine = Machine;
    let mut sate_machine = StateMachine::new();

    println!("my_machine = {my_machine:?}");
}
