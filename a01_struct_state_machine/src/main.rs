// This is our state machine.
struct BottleFillingMachine<S> {
    shared_value: usize,
    state: S,
}

// The following states can be the 'S' in StateMachine<S>
struct Waiting {
    waiting_time: std::time::Duration,
}

struct Filling {
    rate: usize,
}

struct Done;

// Our Machine starts in the 'Waiting' state.
impl BottleFillingMachine<Waiting> {
    fn new(shared_value: usize) -> Self {
        BottleFillingMachine {
            shared_value: shared_value,
            state: Waiting {
                waiting_time: std::time::Duration::new(0, 0),
            },
        }
    }
}

// The following are the defined transitions between states.
impl From<BottleFillingMachine<Waiting>> for BottleFillingMachine<Filling> {
    fn from(val: BottleFillingMachine<Waiting>) -> BottleFillingMachine<Filling> {
        BottleFillingMachine {
            shared_value: val.shared_value,
            state: Filling { rate: 1 },
        }
    }
}

impl From<BottleFillingMachine<Filling>> for BottleFillingMachine<Done> {
    fn from(val: BottleFillingMachine<Filling>) -> BottleFillingMachine<Done> {
        BottleFillingMachine {
            shared_value: val.shared_value,
            state: Done,
        }
    }
}

impl From<BottleFillingMachine<Done>> for BottleFillingMachine<Waiting> {
    fn from(val: BottleFillingMachine<Done>) -> BottleFillingMachine<Waiting> {
        BottleFillingMachine {
            shared_value: val.shared_value,
            state: Waiting {
                waiting_time: std::time::Duration::new(0, 0),
            },
        }
    }
}

// You can play around in this function.
fn main() {
    let bottle_filler = BottleFillingMachine::new(0);

    // (Mock) Check on some shared and state-specific values
    assert_eq!(
        bottle_filler.state.waiting_time,
        std::time::Duration::new(0, 0)
    );
    assert_eq!(bottle_filler.shared_value, 0);

    // Transition
    let bottle_filler = BottleFillingMachine::<Filling>::from(bottle_filler);

    // Can't do this anymore, it's been consumed!:
    // assert_eq!(bottle_filler.state.waiting_time, std::time::Duration::new(0, 0));

    let bottle_filler = BottleFillingMachine::<Done>::from(bottle_filler);
}
