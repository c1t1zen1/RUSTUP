//! Running the swarm: one thread per bot, one shared floor.
use crate::config::Config;
// Bring in what you need: serde::Serialize, std::thread, std::sync::{Arc, Mutex}.

/// What one bot did. Every field comes out the same on every run.
// TODO: also derive Serialize.
#[derive(Debug, Clone, PartialEq)]
pub struct BotReport {
    pub name: String,
    /// Where it stopped, `[x, y]`.
    pub end: (usize, usize),
    /// Moves that went somewhere.
    pub moves: u32,
    /// Moves that hit the edge of the floor, so the bot stayed put.
    pub bumps: u32,
}

/// What the whole swarm did.
// TODO: also derive Serialize.
#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    /// One entry per bot, sorted by name.
    pub bots: Vec<BotReport>,
    /// Units of dirt removed, all bots together.
    pub cleaned: u32,
    /// Units of dirt still on the floor.
    pub dirt_left: u32,
    /// The floor at the end, same shape as the config's.
    pub floor: Vec<Vec<u32>>,
}

impl Report {
    /// The report as pretty-printed JSON (`serde_json::to_string_pretty`).
    pub fn to_json(&self) -> String {
        todo!("serde_json::to_string_pretty, for {} bots", self.bots.len())
    }
}

// Helper functions are welcome here. One bot's whole walk makes a good one.

/// Run the swarm. Each bot gets its own thread, and they all share one floor
/// behind an `Arc<Mutex<..>>`.
///
/// The rules for each bot:
/// - it cleans one unit from its start tile, if there's any dirt there
/// - for each letter of its route it tries to move one tile
/// - a move that would leave the floor is a bump: the bot stays put and cleans nothing
/// - a move that works cleans one unit from the new tile, if there's any dirt there
///
/// Which bot gets the last unit on a tile two bots visit depends on which
/// thread locks first, so the report doesn't say who cleaned what. The totals
/// and the final floor come out the same every time.
pub fn run(config: &Config) -> Report {
    todo!(
        "one thread per bot ({} bots), one shared floor",
        config.bots.len()
    )
}
