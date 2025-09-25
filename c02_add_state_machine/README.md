# Result

```bash
❯ cargo r
   Compiling c02_add_state_machine v0.1.0 (/home/y/my_project/Rust_Lang/9999/pretty_state_machine_patterns_in_rust/c02_add_state_machine)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.77s
     Running `target/debug/c02_add_state_machine`
Creating BottleFillingMachine in Waiting state...
Debug #1: BottleFillingMachine { state: Waiting(Waiting { waiting_time: 0ns, shared_value: 0 }) }
Debug #2: Waiting time = 0ns
Debug #3: Shared value = 0
Transitioning: Waiting → Filling
Debug #4: BottleFillingMachine { state: Filling(Filling { rate: 1, shared_value: 1 }) }
Debug #5: Filling rate = 1
Debug #6: Shared value = 1
Debug #7: Is rate > 0 ? true
Transitioning: Filling → Done
Debug #8: BottleFillingMachine { state: Done(Done { bottles_filled: 42, shared_value: 11 }) }
Debug #9: Bottles filled = 42
Debug #10: Shared value = 11
Debug #11: Is final count > 0 ? true
Transitioning: Done → Waiting
Debug #12: BottleFillingMachine { state: Waiting(Waiting { waiting_time: 0ns, shared_value: 111 }) }
Debug #13: Waiting time reset = 0ns
Debug #14: Shared value after reset = 111
Debug #15: Machine discriminant = Discriminant(0)
Debug #16: Type check = true
Debug #17: Is Filling? false
Debug #18: Is Done? false
Debug #19: Full debug dump = BottleFillingMachine { state: Waiting(Waiting { waiting_time: 0ns, shared_value: 111 }) }
Debug #20: Shared value (via match) = 111

=== NEW STATES DEMONSTRATION ===
Transitioning: Filling → Done
Invalid transition to Done
Debug #22: BottleFillingMachine { state: Waiting(Waiting { waiting_time: 0ns, shared_value: 111 }) }
Transitioning: Done → Cleaning
Invalid transition to Cleaning
Debug #23: BottleFillingMachine { state: Waiting(Waiting { waiting_time: 0ns, shared_value: 111 }) }
Transitioning: Cleaning → Maintenance
Invalid transition to Maintenance
Debug #26: BottleFillingMachine { state: Waiting(Waiting { waiting_time: 0ns, shared_value: 111 }) }
Transitioning: Any → Error
Debug #29: BottleFillingMachine { state: Error(Error { error_code: "VALVE_STUCK", shared_value: 1111 }) }
Debug #30: Error code = VALVE_STUCK
Debug #31: Shared value = 1111
Transitioning: Error → Waiting
Debug #32: BottleFillingMachine { state: Waiting(Waiting { waiting_time: 0ns, shared_value: 1611 }) }
Debug #33: Recovered to Waiting, shared value = 1611
Transitioning: Waiting → Filling
Transitioning: Any → Error
Debug #34: BottleFillingMachine { state: Error(Error { error_code: "PUMP_FAILURE", shared_value: 2612 }) }
Debug #35: Error from Filling state: PUMP_FAILURE
Debug #36: Program finished with new states!

```
