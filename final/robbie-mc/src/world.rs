//! The grid Robbie drives around on.

use crate::McError;

/// One square of the map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    /// `.` open floor.
    Floor,
    /// `#` a wall. Bots bump into it.
    Wall,
    /// `P` a spare part waiting to be picked up.
    Part,
}

impl Tile {
    /// The tile a map character stands for, if any.
    pub fn from_char(ch: char) -> Option<Tile> {
        match ch {
            '.' => Some(Tile::Floor),
            '#' => Some(Tile::Wall),
            'P' => Some(Tile::Part),
            _ => None,
        }
    }

    /// The map character for this tile.
    pub fn to_char(self) -> char {
        match self {
            Tile::Floor => '.',
            Tile::Wall => '#',
            Tile::Part => 'P',
        }
    }
}

/// A rectangular grid of tiles. `(x, y)` is column `x` of row `y`, both
/// counting from 0, so `(0, 0)` is the top left and y grows going South.
#[derive(Debug, Clone, PartialEq)]
pub struct World {
    // Your design. A Vec<Vec<Tile>> of rows is a fine start.
    rows: Vec<Vec<Tile>>,
}

impl World {
    /// Reads a map, one row per line.
    ///
    /// Spaces at the end of a line are ignored and blank lines are skipped.
    /// Every other line must be as long as the first row. Errors give line
    /// numbers in the text as written, counting from 1.
    ///
    /// ```
    /// let world = robbie_mc::World::parse("#####\n#.P.#\n#####").unwrap();
    /// assert_eq!((world.width(), world.height()), (5, 3));
    /// assert_eq!(world.parts_left(), 1);
    /// ```
    pub fn parse(map: &str) -> Result<World, McError> {
        todo!(
            "read the map, one row per line ({} lines)",
            map.lines().count()
        )
    }

    /// How many columns.
    pub fn width(&self) -> usize {
        todo!()
    }

    /// How many rows.
    pub fn height(&self) -> usize {
        todo!()
    }

    /// The tile at `(x, y)`, or `None` off the edge of the map.
    pub fn tile(&self, x: usize, y: usize) -> Option<Tile> {
        todo!("the tile at ({x}, {y}), or None off the map")
    }

    /// Changes the tile at `(x, y)`. Does nothing off the edge of the map.
    pub fn set(&mut self, x: usize, y: usize, tile: Tile) {
        todo!("put {tile:?} at ({x}, {y})")
    }

    /// How many parts are still on the floor.
    pub fn parts_left(&self) -> usize {
        todo!()
    }

    /// The map drawn back as text, one line per row.
    pub fn render(&self) -> String {
        todo!()
    }
}
