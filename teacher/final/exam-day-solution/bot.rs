//! A single Robbie: where he is, which way he faces, and what he's done.

use crate::{Command, Tile, World};

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
        match s.to_uppercase().as_str() {
            "N" => Some(Facing::North),
            "E" => Some(Facing::East),
            "S" => Some(Facing::South),
            "W" => Some(Facing::West),
            _ => None,
        }
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
        match self {
            Facing::North => Facing::West,
            Facing::West => Facing::South,
            Facing::South => Facing::East,
            Facing::East => Facing::North,
        }
    }

    /// A quarter turn to the right.
    pub fn right(self) -> Facing {
        self.left().left().left()
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

    /// The tile in front of the bot, or `None` if that's off the map.
    fn ahead(&self, world: &World) -> Option<(usize, usize)> {
        let (x, y) = (self.x, self.y);
        let next = match self.facing {
            Facing::North => (Some(x), y.checked_sub(1)),
            Facing::South => (Some(x), y.checked_add(1)),
            Facing::West => (x.checked_sub(1), Some(y)),
            Facing::East => (x.checked_add(1), Some(y)),
        };
        match next {
            (Some(nx), Some(ny)) if nx < world.width() && ny < world.height() => Some((nx, ny)),
            _ => None,
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
    /// - `Drop` puts one carried part on the bot's tile, if the tile is plain
    ///   floor, for free.
    /// - Moving onto a charger refills the battery to its capacity.
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
        if self.flat {
            return;
        }
        match cmd {
            Command::Forward(n) => {
                for _ in 0..n {
                    if self.battery == 0 {
                        self.flat = true;
                        return;
                    }
                    self.battery -= 1;
                    match self.ahead(world) {
                        Some((nx, ny)) if world.tile(nx, ny) != Some(Tile::Wall) => {
                            self.x = nx;
                            self.y = ny;
                            self.steps += 1;
                            if world.tile(nx, ny) == Some(Tile::Charger) {
                                self.battery = self.capacity;
                            }
                        }
                        _ => {
                            self.bumps += 1;
                            return;
                        }
                    }
                }
            }
            Command::Left => self.facing = self.facing.left(),
            Command::Right => self.facing = self.facing.right(),
            Command::Pick => {
                if world.tile(self.x, self.y) == Some(Tile::Part) {
                    world.set(self.x, self.y, Tile::Floor);
                    self.parts += 1;
                }
            }
            Command::Wait => {}
            Command::Drop => {
                if self.parts > 0 && world.tile(self.x, self.y) == Some(Tile::Floor) {
                    world.set(self.x, self.y, Tile::Part);
                    self.parts -= 1;
                }
            }
        }
    }
}
