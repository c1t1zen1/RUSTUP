# Final Project: Robbie Mission Control

Robbie is 95% rust. He can still drive, turn and pick things up, but he needs someone to plan his day. You're building that someone: a Rust library and command-line program that loads a grid world and a crew of Robbies, runs each bot's script, and prints a mission report.

The types, signatures and doc comments are in `src/`. Every function body is a `todo!()`. The rules below are the spec: the hidden tests check every one of them.

## The world

A map is text, one row per line:

| Character | Tile | What it does |
| --- | --- | --- |
| `.` | `Tile::Floor` | Open floor |
| `#` | `Tile::Wall` | Bots bump into it |
| `P` | `Tile::Part` | A spare part to pick up |

- `(x, y)` is column `x` of row `y`, both from 0. `(0, 0)` is the top left, and y grows going South.
- Spaces at the end of a line are ignored, and blank lines are skipped.
- Every row must be as long as the first one.
- Errors give line and column numbers from 1, counting every line of the text as written, blank ones included.

## The command language

One command per line, upper or lower case, spaces around it ignored. Blank lines and lines starting with `#` are skipped.

| Command | Battery | What it does |
| --- | --- | --- |
| `forward` or `forward N` | 1 per tile | Drive N tiles (N from 1 up, default 1) |
| `left`, `right` | free | A quarter turn on the spot |
| `pick` | free | Take the part on this tile, if there is one |
| `wait` | free | Nothing |

Driving, one tile at a time:

1. A bot with a battery of 0 that tries to move goes **flat**. A flat bot ignores every command after that, turns and picks included.
2. Otherwise the move costs 1 battery.
3. If the tile ahead is a wall or off the map, that's a **bump**: the bot stays put, and the rest of that `forward` is dropped.
4. Otherwise the bot moves, and that's one **step**.

## Missions

A mission file is JSON:

```json
{
  "map": "#####\n#..P#\n#####",
  "bots": [
    { "name": "Robbie", "start": [1, 1], "facing": "E", "battery": 10,
      "script": "forward 2\npick" }
  ]
}
```

Before anything moves, `Mission::prepare` checks, in this order, and returns the first problem:

1. The map.
2. Then each bot in turn: a name already used (names are case-sensitive), a start off the map, a start inside a wall, a facing that isn't `N`, `E`, `S` or `W` (either case), then its script.

Starting on a part is fine. A mission with no bots is fine too.

`run_mission` runs the bots one at a time, in order, each on the world the last one left. `run_fleet` runs every bot on its own thread at once, sharing one world. Bots never block each other, so every bot ends in the same place either way. When two bots reach the same part, which one gets it can change from run to run, but the totals never do.

The `Report` lists the bots in mission order, the parts collected (the sum of what the bots carry), the parts left on the map, and the map as it ends.

## Errors, not panics

Every problem in a map, script, mission file or command line is an `McError`, and the messages in `src/error.rs` are part of the spec. The library never panics on bad input: no `unwrap`, `expect` or indexing that can go out of bounds in `src/` outside of tests.

## The command line

```
cargo run -- missions/studio.json            # one bot at a time
cargo run -- missions/studio.json --fleet    # every bot on its own thread
cargo run -- missions/studio.json --json     # the report as JSON
cargo run -- missions/warehouse.txt missions/tour.txt
```

With a map and a script, Robbie starts on the first tile that isn't a wall (left to right, top to bottom), facing East, with a battery of 50. The logic lives in `run_cli` so it can be tested. `main.rs` is already written: it prints the result, or `error: ...` and exits with code 1.

## What to hand in

- Every `todo!()` replaced, `cargo test` green.
- At least **10 passing tests of your own** in `tests/mine.rs`. Test the rules, not the examples.
- A doc comment on every public item, and an example on the ones people will call first. `cargo doc --open` to check.
- This README updated with anything you added.

## How it's graded

The grader builds it, runs the hidden tests, and checks your tests, lint, format and docs:

| Check | Points |
| --- | --- |
| `cargo build --release` works (if not, grading stops here) | 15 |
| Hidden tests, as a share passed (the exam-day tests are part of these) | 40 |
| Your own tests: 10 or more passing | 10 |
| `cargo clippy --all-targets -- -D warnings`, minus 1 per warning | 10 |
| `cargo fmt --check` | 5 |
| Docs: `cargo doc` builds, public items documented, a README | 5 |
| Code review by your teacher | 15 |
| Stretch goal | up to +5 |

Run the same grader yourself, any time, from the repository root:

```
cargo run --manifest-path grader/Cargo.toml -- final/robbie-mc
```

Without the hidden tests it scores the visible ones instead, so you'll see every other number exactly as your teacher will.

## Exam day

On exam day you'll be given two new features to add in class. They build on everything above, so a clean design now makes exam day easy. Keep `Tile`, `Command` and `Bot::run` easy to extend.

## Stretch goals (pick one, up to +5)

- **Robbie Arena:** build the library for the web with `wasm-pack` and expose `run_mission_json(&str) -> String`. The course site's Robbie Arena page runs missions in the browser; show yours runs the same ones.
- **A terminal view:** draw the mission step by step in the terminal with `ratatui`.
- **Async fleet:** a `run_fleet_async` with tokio tasks instead of threads, giving the same report.
