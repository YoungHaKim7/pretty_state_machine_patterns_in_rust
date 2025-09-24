# Result

```bash
== Bottle Filling Machine Demo ===
Initial state:
  Machine: BottleFillingMachine { shared_value: 42, state: Waiting { waiting_time: 0ns, shared_value: 42 } }
  Shared value: 42
  Waiting time: 0ns
  State shared value: 42

--- Transitioning to Filling state ---
Filling state:
  Machine: BottleFillingMachine { shared_value: 42, state: Filling { rate: 1, shared_value: 42 } }
  Shared value: 42
  Filling rate: 1
  State shared value: 42

--- Transitioning to Done state ---
Done state:
  Machine: BottleFillingMachine { shared_value: 42, state: Done }
  Shared value: 42
  Is done: true
  State shared value: 0

--- Transitioning back to Waiting state ---
Back to Waiting state:
  Machine: BottleFillingMachine { shared_value: 42, state: Waiting { waiting_time: 0ns, shared_value: 42 } }
  Shared value: 42
  Waiting time: 0ns
  State shared value: 42

--- Demonstrating state-specific methods ---
Standalone Waiting state: Waiting { waiting_time: 0ns, shared_value: 0 }
Converted to Filling state: Filling { rate: 1, shared_value: 0 }

=== Demo Complete ===
The state machine successfully demonstrates:
  ✓ Type-safe state transitions
  ✓ Shared state preservation
  ✓ Modular, organized code structure
  ✓ Clear separation of concerns
```

