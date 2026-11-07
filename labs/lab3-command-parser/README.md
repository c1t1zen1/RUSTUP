# Lab 3: Robbie's Command Parser

Robbie takes orders now. Type `forward 3` and he rolls three squares. Type `dance` and he tilts his head, because he doesn't know how yet (Session 6, maybe).

All the types are written for you in `src/lib.rs`: a `Direction` enum, a `Command` enum that carries data, and a `Robot` struct. Your job is the five `todo!()`s, in order. Each step has its own tests.

| Step | What you write | Check it |
| --- | --- | --- |
| 1 | `Robot::new()` | `cargo test step1` |
| 2 | `Direction::turn_left` and `turn_right` with `match` | `cargo test step2` |
| 3 | `parse(line) -> Option<Command>` | `cargo test step3` |
| 4 | `Robot::apply(&mut self, cmd)` | `cargo test step4` |
| 5 | `Robot::status(&self)` | `cargo test step5` |

When all nine tests pass:

```
cargo run
> forward 3
Moved 3
> left
Turned left, now facing West
> say I am a robot
Robbie says: I am a robot
> report
(0, 3) facing West, battery 97%
```

Rules for today:

- Use `match`, not a chain of `if`s, for the directions and the commands.
- In `parse`, use `?` at least once. `rest.parse().ok()?` is the classic.
- `cargo clippy -- -D warnings` must be clean before you call it done.

Finished early? Add a `Back(u32)` command, then a `Spin` that turns him right four times and says "Wheee".
