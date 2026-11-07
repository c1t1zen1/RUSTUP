//! Robbie himself: where he is, which way he faces, and what he does with a Command.
use crate::parse::Command;
use std::fmt;

/// How far Robbie may roll from the middle of the studio floor, in any direction.
pub const GRID_LIMIT: i32 = 10;

/// Which way Robbie is facing. North is up the grid (y grows).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_left(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn turn_right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    /// One step this way, as (dx, dy).
    pub fn step(self) -> (i32, i32) {
        match self {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        }
    }
}

/// Why Robbie refused a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RobotError {
    LowBattery { needed: u32, left: u32 },
    OutOfBounds { x: i32, y: i32 },
}

impl fmt::Display for RobotError {
    /// "needs 5% battery, has 2%" and "would leave the grid at (0, 11)"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("match on {self:?} and write! the message into {f:p}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Robot {
    pub x: i32,
    pub y: i32,
    pub facing: Direction,
    pub battery: u32,
}

impl Robot {
    pub fn new() -> Robot {
        Robot {
            x: 0,
            y: 0,
            facing: Direction::North,
            battery: 100,
        }
    }

    pub fn status(&self) -> String {
        format!(
            "({}, {}) facing {:?}, battery {}%",
            self.x, self.y, self.facing, self.battery
        )
    }

    /// Carry out one command.
    /// - Forward(n) and Back(n) roll n squares (Back rolls backwards without turning) and use n battery.
    ///   Check the battery first: n more than he has is LowBattery { needed: n, left }.
    ///   Then check the grid: if x or y would end up beyond GRID_LIMIT either way,
    ///   it's OutOfBounds { x, y } with where he would have landed.
    ///   A refused move changes nothing at all.
    /// - Left and Right turn him. Charge sets the battery to 100. These return Ok(None).
    /// - Say(text) returns Ok(Some("Robbie says: text")). Report returns Ok(Some(self.status())).
    pub fn apply(&mut self, cmd: &Command) -> Result<Option<String>, RobotError> {
        todo!("match on {cmd:?} and update {self:?}")
    }
}

impl Default for Robot {
    fn default() -> Self {
        Robot::new()
    }
}
