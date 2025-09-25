#[derive(Debug)]
enum State {
    Waiting(Waiting),
    Filling(Filling),
    Done(Done),
}

#[derive(Debug)]
struct Waiting {
    start_value: String,
}
#[derive(Debug)]
struct Filling {
    interm_value: Vec<String>,
}
#[derive(Debug)]
struct Done {
    final_value: usize,
}

#[derive(Debug)]
struct StateMachine {
    some_unrelated_value: usize,
    state: State,
}

impl StateMachine {
    fn new(start: String) -> Self {
        println!("Creating StateMachine in Waiting state...");
        Self {
            some_unrelated_value: 0,
            state: State::Waiting(Waiting { start_value: start }),
        }
    }

    fn to_filling(self) -> Self {
        println!("Transitioning from Waiting → Filling...");
        match self.state {
            State::Waiting(w) => {
                let words: Vec<String> =
                    w.start_value.split_whitespace().map(|x| x.into()).collect();
                println!("Words parsed: {:?}", words);
                StateMachine {
                    some_unrelated_value: self.some_unrelated_value + 1,
                    state: State::Filling(Filling {
                        interm_value: words,
                    }),
                }
            }
            _ => {
                println!("Invalid transition to Filling!");
                self
            }
        }
    }

    fn to_done(self) -> Self {
        println!("Transitioning from Filling → Done...");
        match self.state {
            State::Filling(f) => {
                let len = f.interm_value.len();
                println!("Word count computed: {}", len);
                StateMachine {
                    some_unrelated_value: self.some_unrelated_value + 10,
                    state: State::Done(Done { final_value: len }),
                }
            }
            _ => {
                println!("Invalid transition to Done!");
                self
            }
        }
    }
}

fn main() {
    let machine = StateMachine::new("Blah blah blah testing state machine enum".into());

    println!("Debug #1: Machine created: {:?}", machine);

    // Transition 1
    let machine = machine.to_filling();
    println!("Debug #2: After to_filling: {:?}", machine);

    // Access the interm value
    if let State::Filling(f) = &machine.state {
        println!("Debug #3: Interm value: {:?}", f.interm_value);
        println!("Debug #4: First word = {:?}", f.interm_value.get(0));
    }

    // Transition 2
    let machine = machine.to_done();
    println!("Debug #5: After to_done: {:?}", machine);

    // Access the final value
    if let State::Done(d) = &machine.state {
        println!("Debug #6: Final value (word count) = {}", d.final_value);
    }

    // Show unrelated value
    println!(
        "Debug #7: Unrelated value = {}",
        machine.some_unrelated_value
    );

    // More debug prints
    println!(
        "Debug #8: Is Done state? {}",
        matches!(machine.state, State::Done(_))
    );
    println!(
        "Debug #9: Machine state type = {:?}",
        std::mem::discriminant(&machine.state)
    );
    println!("Debug #10: Full Debug dump = {:?}", machine);
    println!("Debug #11: Program finished successfully.");
}
