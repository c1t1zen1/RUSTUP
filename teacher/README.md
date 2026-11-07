# Teacher's kit

This folder holds the answers, so it's kept out of the published course site (GitHub Pages only serves the `docs/` folder). It is still readable by anyone who browses the repository on GitHub.

| Path | What it is |
| --- | --- |
| `s1/notebook.html` | Session 1 Teacher's Notebook: run of show, deeper notes, common mistakes, lab and homework solutions, Quiz 1 answer key |
| `s1/hw1-solution-lib.rs` | Reference solution for Homework 1 (drop it in as `src/lib.rs`) |
| `grading/hw1-hidden-tests.rs` | Hidden edge-case tests for Homework 1 (copy into the student's `tests/` folder before grading) |
| `s2/notebook.html` | Session 2 Teacher's Notebook, with the Quiz 2 answer key |
| `s2/lab2-solutions/` | Fixed versions of all seven Lab 2 puzzles |
| `s2/hw2-solution-lib.rs` | Reference solution for Homework 2 |
| `grading/hw2-hidden-tests.rs` | Hidden edge-case tests for Homework 2 |
| `s3/notebook.html` | Session 3 Teacher's Notebook, with the Quiz 3 and Checkpoint 1 answer keys |
| `s3/lab3-solution-lib.rs` | Reference solution for Lab 3 (drop it in as `src/lib.rs`) |
| `s3/hw3-solution/` | Reference solution for Homework 3 (`lib.rs`, `parse.rs`, `robot.rs` go in `src/`) |
| `grading/hw3-hidden-tests.rs` | Hidden edge-case tests for Homework 3 |
| `s4/notebook.html` | Session 4 Teacher's Notebook, with the Quiz 4 answer key |
| `s4/lab4-solution-lib.rs` | Reference solution for Lab 4 (drop it in as `src/lib.rs`) |
| `s4/hw4-solution/` | Reference solution for Homework 4 (`lib.rs`, `bots.rs`, `garage.rs` go in `src/`) |
| `grading/hw4-hidden-tests.rs` | Hidden edge-case tests for Homework 4 |
| `s5/notebook.html` | Session 5 Teacher's Notebook, with the Quiz 5 and Checkpoint 2 answer keys |
| `s5/lab5-solution-lib.rs` | Reference solution for Lab 5 (drop it in as `src/lib.rs`) |
| `s5/hw5-solution/` | Reference solution for Homework 5 (`lib.rs`, `config.rs`, `sim.rs` go in `src/`) |
| `grading/hw5-hidden-tests.rs` | Hidden edge-case tests for Homework 5 |
| `s6/notebook.html` | Session 6 Teacher's Notebook: Lab 6 solution, the final project, the exam-day run of show, grading, and the Quiz 6 answer key |
| `s6/lab6-solution-lib.rs` | Reference solution for Lab 6 (drop it in as `src/lib.rs`, then fill the `[package]` fields) |
| `final/solution/` | Reference solution for the final project (every file goes in `src/`) |
| `final/solution-mine.rs` | 11 example student tests, each testing one rule |
| `final/exam-day.md` | The exam-day handout: two new features, 60 minutes |
| `final/exam-day-solution/` | `world.rs`, `command.rs` and `bot.rs` with both exam-day features added |
| `grading/final-hidden-tests.rs` | 17 hidden tests for the final project |
| `grading/final-examday-tests.rs` | 8 hidden tests for the exam-day features |

## Grading

Every homework and the final are graded by `robbie-grader` (see `grader/README.md`). For the final, after exam day:

```
cargo run --release --manifest-path grader/Cargo.toml -- path/to/student/robbie-mc \
  --hidden teacher/grading/final-hidden-tests.rs \
  --hidden teacher/grading/final-examday-tests.rs
```

Then open the student's `grade.json` in the Review Lab (`docs/review-lab.html`), score the code review, and run the command it writes, which adds `--review N --stretch N`.

If the exam-day features must stay a surprise, keep this folder in a private repository for your cohort: anything here can be read on GitHub.

## Opening the notebook

The notebook uses the site's styles and Robbie, so serve the repository root rather than double-clicking the file:

```
python3 -m http.server 8080
```

Then open http://localhost:8080/teacher/s1/notebook.html (or s2 to s6).
