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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RobotError::LowBattery { needed, left } => {
                write!(f, "needs {needed}% battery, has {left}%")
            }
            RobotError::OutOfBounds { x, y } => write!(f, "would leave the grid at ({x}, {y})"),
        }
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

    /// Carry out one command. Say and Report return a line to print; the rest return None.
    /// A refused move changes nothing.
    pub fn apply(&mut self, cmd: &Command) -> Result<Option<String>, RobotError> {
        match cmd {
            Command::Forward(n) => self.roll(*n, 1).map(|_| None),
            Command::Back(n) => self.roll(*n, -1).map(|_| None),
            Command::Left => {
                self.facing = self.facing.turn_left();
                Ok(None)
            }
            Command::Right => {
                self.facing = self.facing.turn_right();
                Ok(None)
            }
            Command::Say(text) => Ok(Some(format!("Robbie says: {text}"))),
            Command::Charge => {
                self.battery = 100;
                Ok(None)
            }
            Command::Report => Ok(Some(self.status())),
        }
    }

    fn roll(&mut self, n: u32, sign: i32) -> Result<(), RobotError> {
        if n > self.battery {
            return Err(RobotError::LowBattery {
                needed: n,
                left: self.battery,
            });
        }
        let (dx, dy) = self.facing.step();
        let dist = n as i32 * sign;
        let (x, y) = (self.x + dx * dist, self.y + dy * dist);
        if x.abs() > GRID_LIMIT || y.abs() > GRID_LIMIT {
            return Err(RobotError::OutOfBounds { x, y });
        }
        self.x = x;
        self.y = y;
        self.battery -= n;
        Ok(())
    }
}

impl Default for Robot {
    fn default() -> Self {
        Robot::new()
    }
}
