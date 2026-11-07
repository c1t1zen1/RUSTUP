//! Lab 3: Robbie's Command Parser.
//! The types are done. Replace each todo!() in order, and run `cargo test step1`,
//! then `cargo test step2`, and so on. When everything passes, `cargo run` and drive him.

/// Which way Robbie is facing. North is up the grid (y grows).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Step 2: a quarter turn anticlockwise. North turns left to West.
    pub fn turn_left(self) -> Direction {
        todo!("match on {self:?} and return the next direction anticlockwise")
    }

    /// Step 2: a quarter turn clockwise. North turns right to East.
    pub fn turn_right(self) -> Direction {
        todo!("match on {self:?} and return the next direction clockwise")
    }
}

/// Everything Robbie understands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Forward(u32),
    Left,
    Right,
    Say(String),
    Charge,
    Report,
}

/// Robbie's position, heading and battery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Robot {
    pub x: i32,
    pub y: i32,
    pub facing: Direction,
    pub battery: u32,
}

impl Robot {
    /// Step 1: a fresh Robbie at (0, 0), facing North, battery 100.
    pub fn new() -> Robot {
        todo!("build a Robot with struct literal syntax")
    }

    /// Step 4: carry out one command and say what happened.
    /// - Forward(n): move n squares the way he's facing and use n battery.
    ///   If n is more than the battery left, don't move; return "Not enough battery to move n".
    /// - Left / Right: turn. Say(text): return "Robbie says: text".
    /// - Charge: battery back to 100. Report: return self.status().
    pub fn apply(&mut self, cmd: Command) -> String {
        todo!("match on {cmd:?} and update {self:?}")
    }

    /// Step 5: one line about where Robbie is, like "(-4, 0) facing West, battery 96%".
    /// Hint: {:?} prints a Direction's name.
    pub fn status(&self) -> String {
        todo!("format {self:?} as one line")
    }
}

impl Default for Robot {
    fn default() -> Self {
        Robot::new()
    }
}

/// Step 3: turn a typed line into a Command, or None if it makes no sense.
/// Words are case-insensitive and extra spaces don't matter.
/// "forward 3" needs a whole number; "say" needs some text after it.
/// Hint: line.trim(), line.split_once(' '), word.to_lowercase(), and rest.parse().ok()?
pub fn parse(line: &str) -> Option<Command> {
    todo!("parse {line:?}")
}
