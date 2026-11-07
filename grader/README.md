# robbie-grader

Scores a RUSTUP final project exactly the way the course does. Students can run it on their own work any time; teachers add the hidden tests.

```
cargo run --release --manifest-path grader/Cargo.toml -- check hw3     # a homework
cargo run --release --manifest-path grader/Cargo.toml -- check final   # the final project
```

Run those from the course folder. Any project path works too, in place of `check NAME`.

It runs, in order:

| Check | Final project | Homework 1 to 4 | Homework 5 | How |
| --- | --- | --- | --- | --- |
| Build | 15 | 2 | 2 | `cargo build --release`. If it fails, grading stops |
| Visible tests | | 3 | 2 | Share of `tests/visible.rs` passing |
| Hidden tests | 40 | 3 | 3 | Each `--hidden` file is copied into `tests/` for one run, then removed. For the final, the visible tests stand in until the hidden ones are added |
| Your tests | 10 | | 1 | Passing tests in `tests/mine.rs`: one point each up to 10 for the final, three or more for Homework 5 |
| Clippy | 10 | 1 | 1 | `cargo clippy --all-targets`. Final: minus 1 per distinct warning. Homework: clean or nothing |
| Format | 5 | 1 | 1 | `cargo fmt --check` |
| Docs | 5 | | | `cargo doc` builds (2), share of public items with a `///` comment (2), a README (1) |
| Code review | 15 | | | Scored by the teacher with `--review N`. The grader lists lines worth a look: `unwrap`, `expect`, `panic!`, `todo!` and `unsafe` in library code |
| Stretch | +5 | | | Scored by the teacher with `--stretch N` |

The rubric is picked from the folder name: `hw1-...` to `hw5-...` are homework, anything else is graded as the final project.

A test run that goes over five minutes is stopped, so a hung thread costs those tests, not the whole grade.

It prints a scorecard and writes `grade.json` into the project folder. Drop that file on the course site's Review Lab page to do the code review side by side with the flags.

## Teacher's run, after exam day

```
cargo run --release --manifest-path grader/Cargo.toml -- path/to/student/robbie-mc \
  --hidden teacher/grading/final-hidden-tests.rs \
  --hidden teacher/grading/final-examday-tests.rs
```

Each hidden file is compiled on its own, so a student who didn't finish an exam-day feature loses those tests and nothing else.

The grader only uses the standard library, so it builds offline.
