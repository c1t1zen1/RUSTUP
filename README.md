# RUSTUP

A three-week, six-session Rust master class, taught with Robbie, a robot teaching assistant who rusts a little more every session.

**Live site:** https://c1t1zen1.github.io/RUSTUP/

## What's inside

| Folder | What it holds |
| --- | --- |
| `docs/` | The course website, published by GitHub Pages |
| `docs/sessions/s1/` … `s6/` | Each session: hub page, slides (`presentation.html`), Student Handbook, quiz. Session 6 also has the final project brief and the Robbie Arena |
| `docs/review-lab.html` | Review Lab: reads a grader report, scores a code review, works out the course grade |
| `docs/assets/` | Site CSS and JS, the 3D Robbie model (`models/robbie.glb`) and stills |
| `labs/` | In-class labs, one cargo project each |
| `homework/` | Homework starters with visible tests |
| `final/robbie-mc/` | The final project starter: Robbie Mission Control |
| `grader/` | `robbie-grader`, which builds, tests, lints and scores a homework or the final |
| `teacher/` | Teacher's Notebooks, solutions and hidden tests. Not published to the website |

## Run it locally

The site is plain HTML, CSS and JavaScript with no build step. Serve the folder with any static server, for example:

```
python3 -m http.server 8080
```

Then open http://localhost:8080/docs/. (Opening the pages straight from disk works for most things, but the 3D Robbie needs a server to load his model.)

GitHub Pages settings: deploy from the `main` branch, `/docs` folder.

## Doing the labs and homework

You need Rust installed from https://rustup.rs. Then, inside a lab or homework folder:

```
cargo run          # run it
cargo test         # run the tests
cargo clippy       # lint
cargo fmt --check  # check formatting
```

Homework 5 and later use crates from crates.io (serde, serde_json), so the first build needs an internet connection.

## Grading

From the repository root, any time:

```
cargo run --release --manifest-path grader/Cargo.toml -- check hw3     # a homework
cargo run --release --manifest-path grader/Cargo.toml -- check final   # the final project
```

It prints a scorecard and writes `grade.json`. Open that in the Review Lab to read every flag in plain English.

## Robbie

Robbie v11 was modeled and animated in Blender. The web version is exported from the same file, drawn with a cel-shaded look, and gets rustier with each session's `data-rust` level (0 to 5).

## Status

All six sessions are ready, with Checkpoints 1 and 2, the final project, exam day, the grader, the Robbie Arena and the Review Lab.
