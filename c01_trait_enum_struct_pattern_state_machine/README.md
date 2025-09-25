# Result

```bash
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
Debug #21: Program finished.

```

