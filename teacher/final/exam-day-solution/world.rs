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
    /// `C` a charging pad. Driving onto it refills the battery.
    Charger,
}

impl Tile {
    /// The tile a map character stands for, if any.
    pub fn from_char(ch: char) -> Option<Tile> {
        match ch {
            '.' => Some(Tile::Floor),
            '#' => Some(Tile::Wall),
            'P' => Some(Tile::Part),
            'C' => Some(Tile::Charger),
            _ => None,
        }
    }

    /// The map character for this tile.
    pub fn to_char(self) -> char {
        match self {
            Tile::Floor => '.',
            Tile::Wall => '#',
            Tile::Part => 'P',
            Tile::Charger => 'C',
        }
    }
}

/// A rectangular grid of tiles. `(x, y)` is column `x` of row `y`, both
/// counting from 0, so `(0, 0)` is the top left and y grows going South.
#[derive(Debug, Clone, PartialEq)]
pub struct World {
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
        let mut rows: Vec<Vec<Tile>> = Vec::new();
        for (i, raw) in map.lines().enumerate() {
            let line = raw.trim_end();
            if line.is_empty() {
                continue;
            }
            let mut row = Vec::new();
            for (c, ch) in line.chars().enumerate() {
                match Tile::from_char(ch) {
                    Some(t) => row.push(t),
                    None => {
                        return Err(McError::UnknownTile {
                            line: i + 1,
                            col: c + 1,
                            ch,
                        });
                    }
                }
            }
            if let Some(first) = rows.first()
                && first.len() != row.len()
            {
                return Err(McError::RaggedMap { line: i + 1 });
            }
            rows.push(row);
        }
        if rows.is_empty() {
            return Err(McError::EmptyMap);
        }
        Ok(World { rows })
    }

    /// How many columns.
    pub fn width(&self) -> usize {
        self.rows[0].len()
    }

    /// How many rows.
    pub fn height(&self) -> usize {
        self.rows.len()
    }

    /// The tile at `(x, y)`, or `None` off the edge of the map.
    pub fn tile(&self, x: usize, y: usize) -> Option<Tile> {
        self.rows.get(y).and_then(|row| row.get(x)).copied()
    }

    /// Changes the tile at `(x, y)`. Does nothing off the edge of the map.
    pub fn set(&mut self, x: usize, y: usize, tile: Tile) {
        if let Some(t) = self.rows.get_mut(y).and_then(|row| row.get_mut(x)) {
            *t = tile;
        }
    }

    /// How many parts are still on the floor.
    pub fn parts_left(&self) -> usize {
        self.rows
            .iter()
            .flatten()
            .filter(|t| **t == Tile::Part)
            .count()
    }

    /// The map drawn back as text, one line per row.
    pub fn render(&self) -> String {
        self.rows
            .iter()
            .map(|row| row.iter().map(|t| t.to_char()).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
