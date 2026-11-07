//! The swarm's config file: the floor, the dirt on it, and each bot's route.
use std::fmt;
// You'll need serde's derive macros here, and maybe a HashSet.

/// One bot's plan for the shift.
// TODO: also derive serde's Serialize and Deserialize.
#[derive(Debug, Clone, PartialEq)]
pub struct BotPlan {
    pub name: String,
    /// `[x, y]`: x is the column, y is the row, both from 0 at the top left.
    pub start: (usize, usize),
    /// Moves, one letter each: N (up), E (right), S (down), W (left).
    pub route: String,
}

/// The whole config, exactly as it sits in the JSON file.
// TODO: also derive serde's Serialize and Deserialize.
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    /// Rows of dirt, top row first. `floor[y][x]` is the dirt on one tile.
    pub floor: Vec<Vec<u32>>,
    pub bots: Vec<BotPlan>,
}

/// Everything that can be wrong with a config.
#[derive(Debug, Clone, PartialEq)]
pub enum SimError {
    /// The text isn't valid JSON for a Config. Holds serde_json's message.
    Parse(String),
    /// No rows, or a row with no tiles in it.
    EmptyFloor,
    /// Row `row` has a different length from row 0.
    RaggedFloor { row: usize },
    /// Two bots share a name. Holds the second one's name.
    DuplicateBot(String),
    /// A bot starts outside the floor.
    OffFloor { bot: String },
    /// A route has a letter that isn't N, E, S or W.
    BadMove { bot: String, step: char },
}

/// Messages:
/// - Parse(msg)                          -> "can't read config: {msg}"
/// - EmptyFloor                          -> "the floor is empty"
/// - RaggedFloor { row: 2 }              -> "row 2 is a different length from row 0"
/// - DuplicateBot("Robbie")              -> "two bots are called Robbie"
/// - OffFloor { bot: "Robbie" }          -> "Robbie starts off the floor"
/// - BadMove { bot: "Robbie", step: 'x' } -> "Robbie has a bad move: 'x'"
impl fmt::Display for SimError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = f; // you will write! into f
        todo!("one message per variant, for {self:?}")
    }
}

impl std::error::Error for SimError {}

impl Config {
    /// Read a config from JSON text and check it. Checks run in this order,
    /// and the first problem found is the one returned:
    /// 1. the JSON parses (otherwise `Parse`)
    /// 2. the floor has at least one row, and row 0 at least one tile (`EmptyFloor`)
    /// 3. every row is as long as row 0; the first one that isn't is reported (`RaggedFloor`)
    /// 4. then each bot in order: a name already used, exactly (`DuplicateBot`),
    ///    a start outside the floor (`OffFloor`), then the first bad letter in
    ///    its route (`BadMove`). Letters are uppercase only.
    pub fn from_json(text: &str) -> Result<Config, SimError> {
        todo!(
            "parse with serde_json::from_str, then check ({} bytes)",
            text.len()
        )
    }

    /// The floor's width in tiles.
    pub fn width(&self) -> usize {
        self.floor.first().map_or(0, |row| row.len())
    }

    /// The floor's height in tiles.
    pub fn height(&self) -> usize {
        self.floor.len()
    }
}
