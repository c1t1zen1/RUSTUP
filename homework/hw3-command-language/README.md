# Homework 3: Command Language

In the lab, Robbie learned single commands and shrugged (`None`) at anything he didn't understand. Now he runs whole scripts, and when something goes wrong he says exactly what and on which line. Shrugging is no longer allowed. This is a robot with standards. Rusty standards, but standards.

```
# Robbie's studio tour
say Welcome to the studio!
forward 4
right
forward 3
report
```

The project is split into modules, the way real Rust projects are:

| File | What's in it | Your job |
| --- | --- | --- |
| `src/parse.rs` | `Command`, `ParseError`, `parse_line` | `parse_line` and the error messages |
| `src/robot.rs` | `Direction`, `Robot`, `RobotError` | `Robot::apply` and the error messages |
| `src/lib.rs` | `ScriptError`, `run_script` | `run_script` and the error messages |
| `src/main.rs` | Runs a tour script and a broken one | Nothing, it's done |

Every rule is written in the doc comment above the function you're filling in. Read them: the hidden tests check exactly those rules.

```
cargo test                    # 8 visible tests
cargo clippy -- -D warnings   # no warnings allowed
cargo fmt --check             # formatted the standard way
cargo run                     # the tour, then a script that fails on line 2
```

| Graded on | Points |
| --- | --- |
| Builds | 2 |
| Visible tests pass | 3 |
| Hidden tests pass (case and spaces, zero and negative distances, the grid edge, battery before bounds, comments and line numbers) | 3 |
| clippy clean | 1 |
| fmt clean | 1 |

Hints: `split_once(char::is_whitespace)` splits off the first word, `.parse::<u32>()` gives a `Result`, `.map_err(|_| ...)` changes its error type, and `?` returns early with it. `write!(f, "...")` is how `Display` prints.
