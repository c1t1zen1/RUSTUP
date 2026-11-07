//! A single Robbie: where he is, which way he faces, and what he's done.

use crate::{Command, World};

/// A compass direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    East,
    South,
    West,
}

impl Facing {
    /// Reads `N`, `E`, `S` or `W`, in either case.
    pub fn parse(s: &str) -> Option<Facing> {
        todo!("N, E, S or W, in either case: {s}")
    }

    /// The first letter: `'N'`, `'E'`, `'S'` or `'W'`.
    pub fn letter(self) -> char {
        match self {
            Facing::North => 'N',
            Facing::East => 'E',
            Facing::South => 'S',
            Facing::West => 'W',
        }
    }

    /// A quarter turn to the left.
    pub fn left(self) -> Facing {
        todo!("a quarter turn left from {self:?}")
    }

    /// A quarter turn to the right.
    pub fn right(self) -> Facing {
        todo!("a quarter turn right from {self:?}")
    }
}

/// One bot and everything it's done so far.
#[derive(Debug, Clone, PartialEq)]
pub struct Bot {
    pub name: String,
    pub x: usize,
    pub y: usize,
    pub facing: Facing,
    /// Charge left. Each tile driven, or each bump, uses one.
    pub battery: u32,
    /// The battery it started with: a full charge.
    pub capacity: u32,
    /// Parts it's carrying.
    pub parts: u32,
    /// Tiles it has actually moved.
    pub steps: u32,
    /// Times it drove into a wall or the edge of the map.
    pub bumps: u32,
    /// True once it tried to drive on an empty battery. A flat bot ignores
    /// every command after that.
    pub flat: bool,
}

impl Bot {
    /// A new bot with a full battery, carrying nothing.
    pub fn new(name: &str, x: usize, y: usize, facing: Facing, battery: u32) -> Bot {
        Bot {
            name: name.to_string(),
            x,
            y,
            facing,
            battery,
            capacity: battery,
            parts: 0,
            steps: 0,
            bumps: 0,
            flat: false,
        }
    }

    /// Carries out one command on `world`.
    ///
    /// - `Forward(n)` tries `n` single moves. Each costs one battery. A move
    ///   into a wall or off the map is a bump: it costs the battery, the bot
    ///   stays put, and the rest of that `Forward` is dropped. Trying to move
    ///   with a battery of 0 makes the bot flat.
    /// - `Left` and `Right` turn, for free.
    /// - `Pick` takes the part on the bot's tile, if there is one, for free.
    /// - `Wait` does nothing.
    ///
    /// ```
    /// use robbie_mc::{Bot, Command, Facing, World};
    /// let mut world = World::parse("....P").unwrap();
    /// let mut robbie = Bot::new("Robbie", 0, 0, Facing::East, 10);
    /// robbie.run(&mut world, Command::Forward(9));
    /// assert_eq!((robbie.x, robbie.steps, robbie.bumps, robbie.battery), (4, 4, 1, 5));
    /// robbie.run(&mut world, Command::Pick);
    /// assert_eq!((robbie.parts, world.parts_left()), (1, 0));
    /// ```
    pub fn run(&mut self, world: &mut World, cmd: Command) {
        todo!(
            "carry out {cmd:?} on a {}x{} world",
            world.width(),
            world.height()
        )
    }
}
