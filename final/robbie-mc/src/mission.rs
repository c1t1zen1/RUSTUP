//! Missions: a map, some bots, a script each, and the report at the end.

// You'll want more imports as you go: std::collections::HashSet,
// std::sync::{Arc, Mutex}, std::thread, and more from crate::.
use serde::{Deserialize, Serialize};

use crate::{Bot, Command, McError, World};

/// Every bot in a mission, each with its commands, ready to run.
pub type Crew = Vec<(Bot, Vec<Command>)>;

/// One bot as a mission file describes it.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BotSpec {
    pub name: String,
    /// `[x, y]`, counting from the top left.
    pub start: (usize, usize),
    /// `"N"`, `"E"`, `"S"` or `"W"`.
    pub facing: String,
    pub battery: u32,
    /// The bot's commands, one per line.
    pub script: String,
}

/// A whole mission, as read from JSON.
///
/// ```json
/// { "map": "#####\n#..P#\n#####",
///   "bots": [ { "name": "Robbie", "start": [1, 1], "facing": "E",
///               "battery": 10, "script": "forward 2\npick" } ] }
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Mission {
    pub map: String,
    pub bots: Vec<BotSpec>,
}

/// How one bot finished.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BotReport {
    pub name: String,
    pub end: (usize, usize),
    pub facing: char,
    pub battery: u32,
    pub parts: u32,
    pub steps: u32,
    pub bumps: u32,
    pub flat: bool,
}

/// How the whole mission finished. Bots are listed in mission order.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Report {
    pub bots: Vec<BotReport>,
    pub parts_collected: u32,
    pub parts_left: usize,
    pub map: String,
}

impl Mission {
    /// Reads a mission from JSON. A JSON problem becomes `McError::Json`.
    pub fn from_json(text: &str) -> Result<Mission, McError> {
        todo!("turn {} bytes of JSON into a Mission", text.len())
    }

    /// Checks everything before anything moves, and builds the world and
    /// the bots.
    ///
    /// The map is checked first. Then each bot in order: a repeated name,
    /// a start off the map, a start inside a wall, the facing, then its
    /// script. The first problem found is the one returned.
    pub fn prepare(&self) -> Result<(World, Crew), McError> {
        todo!(
            "check the map, then each of the {} bots, in order",
            self.bots.len()
        )
    }
}

/// Runs a mission one bot at a time, in order: the first bot runs its whole
/// script, then the second starts on the world the first one left.
pub fn run_mission(mission: &Mission) -> Result<Report, McError> {
    todo!("run {} bots, one after another", mission.bots.len())
}

/// Runs a mission with every bot on its own thread, all at once, sharing
/// one world.
///
/// Bots never block each other, so every bot ends in the same place as in
/// `run_mission`. When two bots reach the same part, which one gets it can
/// change from run to run, but the totals never do.
pub fn run_fleet(mission: &Mission) -> Result<Report, McError> {
    todo!("run {} bots, each on its own thread", mission.bots.len())
}

impl Report {
    /// The report as pretty JSON.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}"))
    }

    /// The report as a short table for people.
    ///
    /// ```text
    /// Mission report
    ///   Robbie   at (3, 1) facing E   battery 8   parts 1   steps 2   bumps 0
    /// Parts collected: 1. Parts left: 0.
    /// ```
    pub fn to_text(&self) -> String {
        todo!("the table for {} bots", self.bots.len())
    }
}
