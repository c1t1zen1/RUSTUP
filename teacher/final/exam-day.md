# Final exam day: two new features

Hand this out at the start of the coding part of the exam. Students have 60 minutes. The tests for both features are `teacher/grading/final-examday-tests.rs`; the grader copies them in with a second `--hidden` flag (see `grader/README.md`).

## Feature 1: charging pads

A new tile, `C`, is a charging pad. Add it as `Tile::Charger`.

- `Tile::from_char('C')` is `Some(Tile::Charger)`, and `Tile::Charger.to_char()` is `'C'`.
- Bots drive over a charger like floor.
- **Moving onto** a charger refills the bot's battery to its capacity (the battery it started with). Starting on one, turning on one or waiting on one doesn't.
- `pick` on a charger does nothing.

## Feature 2: the `drop` command

A new command, `drop` (any case, no number), parsed as `Command::Drop`.

- It puts one carried part down on the bot's tile, if the bot carries at least one and the tile is plain floor. The tile becomes `P`.
- Otherwise it does nothing. It never costs battery.
- A flat bot ignores it, like everything else.
- The report's `parts_collected` is still what the bots carry at the end, so a dropped part counts as left on the map.

## What a good answer looks like

Both features are about 15 lines in total, in `world.rs`, `command.rs` and `bot.rs`: a new enum variant each, a new match arm in each `match`, and two `if`s in `Bot::run`. A student whose `run` was one big `match` on the command finds it easy. One who hard-coded `'P'` and `'#'` checks all over the code has a harder afternoon, which is what the code review points are for.

Reference: `teacher/final/exam-day-solution/` has the three changed files.
