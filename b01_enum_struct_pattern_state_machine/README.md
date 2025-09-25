# Result

```bash
Creating StateMachine in Waiting state...
Debug #1: Machine created: StateMachine { some_unrelated_value: 0, state: Waiting(Waiting { start_value: "Blah blah blah testing state machine enum" }
) }
Transitioning from Waiting → Filling...
Words parsed: ["Blah", "blah", "blah", "testing", "state", "machine", "enum"]
Debug #2: After to_filling: StateMachine { some_unrelated_value: 1, state: Filling(Filling { interm_value: ["Blah", "blah", "blah", "testing", "state"
, "machine", "enum"] }) }
Debug #3: Interm value: ["Blah", "blah", "blah", "testing", "state", "machine", "enum"]
Debug #4: First word = Some("Blah")
Transitioning from Filling → Done...
Word count computed: 7
Debug #5: After to_done: StateMachine { some_unrelated_value: 11, state: Done(Done { final_value: 7 }) }
Debug #6: Final value (word count) = 7
Debug #7: Unrelated value = 11
Debug #8: Is Done state? true
Debug #9: Machine state type = Discriminant(2)
Debug #10: Full Debug dump = StateMachine { some_unrelated_value: 11, state: Done(Done { final_value: 7 }) }
Debug #11: Program finished successfully.

```

