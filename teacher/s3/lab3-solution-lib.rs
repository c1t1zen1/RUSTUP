//! Lab 3 reference solution: Robbie's Command Parser.
//! Drop this in as labs/lab3-command-parser/src/lib.rs to check the tests.

/// Which way Robbie is facing. North is up the grid (y grows).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Step 2: a quarter turn anticlockwise.
    pub fn turn_left(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    /// Step 2: a quarter turn clockwise.
    pub fn turn_right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
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
        Robot {
            x: 0,
            y: 0,
            facing: Direction::North,
            battery: 100,
        }
    }

    /// Step 4: carry out one command and say what happened.
    pub fn apply(&mut self, cmd: Command) -> String {
        match cmd {
            Command::Forward(steps) => {
                if steps > self.battery {
                    return format!("Not enough battery to move {steps}");
                }
                let n = steps as i32;
                match self.facing {
                    Direction::North => self.y += n,
                    Direction::South => self.y -= n,
                    Direction::East => self.x += n,
                    Direction::West => self.x -= n,
                }
                self.battery -= steps;
                format!("Moved {steps}")
            }
            Command::Left => {
                self.facing = self.facing.turn_left();
                format!("Turned left, now facing {:?}", self.facing)
            }
            Command::Right => {
                self.facing = self.facing.turn_right();
                format!("Turned right, now facing {:?}", self.facing)
            }
            Command::Say(text) => format!("Robbie says: {text}"),
            Command::Charge => {
                self.battery = 100;
                String::from("Charged to 100%")
            }
            Command::Report => self.status(),
        }
    }

    /// Step 5: one line about where Robbie is.
    pub fn status(&self) -> String {
        format!(
            "({}, {}) facing {:?}, battery {}%",
            self.x, self.y, self.facing, self.battery
        )
    }
}

impl Default for Robot {
    fn default() -> Self {
        Robot::new()
    }
}

/// Step 3: turn a typed line into a Command, or None if it makes no sense.
pub fn parse(line: &str) -> Option<Command> {
    let line = line.trim();
    let (word, rest) = match line.split_once(' ') {
        Some((w, r)) => (w, r.trim()),
        None => (line, ""),
    };
    match word.to_lowercase().as_str() {
        "forward" => {
            let steps: u32 = rest.parse().ok()?;
            Some(Command::Forward(steps))
        }
        "left" => Some(Command::Left),
        "right" => Some(Command::Right),
        "say" if !rest.is_empty() => Some(Command::Say(rest.to_string())),
        "charge" => Some(Command::Charge),
        "report" => Some(Command::Report),
        _ => None,
    }
}
