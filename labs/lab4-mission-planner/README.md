# Lab 4: Robbie's Mission Planner

Robbie has a to-do list for the studio, a battery that lasts about an hour, and knees that squeak. Your job is to turn the list into a plan, using iterators, closures and a `HashMap` instead of hand-written loops wherever you can.

The `Task` struct is written for you in `src/lib.rs`. Fill in the five `todo!()`s, in order. Each step has its own tests.

| Step | What you write | Check it |
| --- | --- | --- |
| 1 | `minutes_left`: `filter`, `map`, `sum` | `cargo test step1` |
| 2 | `todo_list`: `collect` into a `Vec<&Task>`, then `sort_by` | `cargo test step2` |
| 3 | `minutes_by_room`: the `entry` API | `cargo test step3` |
| 4 | `plan`: fit tasks into a time budget | `cargo test step4` |
| 5 | `next_in_room`: `find`, and why the `'a` is there | `cargo test step5` |

When all nine tests pass:

```
cargo run
Robbie's to-do list (125 minutes left):
  [5] find umbrella         10 min  (lobby)
  [4] check the cameras     15 min  (studio)
  ...
One 60-minute shift: ["find umbrella", "check the cameras", "charge batteries"]
Next in the workshop: sort bolts
```

Rules for today:

- Steps 1 and 5 can each be written as one iterator chain. Try that first. Step 3 is a short `for` loop over a `filter`.
- `sort_by` takes a closure that compares two items. `.cmp()` and `.then()` chain the tie-breakers.
- `cargo clippy -- -D warnings` must be clean before you call it done.

Finished early? Add `fn rooms_in_order(tasks: &[Task]) -> Vec<(String, u32)>`, busiest room first, and print it in `main`.
