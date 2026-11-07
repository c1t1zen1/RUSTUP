//! Everything that can go wrong before a mission starts.

use std::fmt;

/// A problem with a map, a script, a mission file or the command line.
/// Once a mission has passed its checks it always runs to the end, so
/// nothing here can happen halfway through.
#[derive(Debug, Clone, PartialEq)]
pub enum McError {
    /// The map has no rows at all.
    EmptyMap,
    /// A map character that isn't a known tile. `line` and `col` count from 1.
    UnknownTile { line: usize, col: usize, ch: char },
    /// A row that isn't as long as the first row. `line` counts from 1.
    RaggedMap { line: usize },
    /// A script line that isn't a command. `line` counts from 1.
    BadCommand { line: usize, text: String },
    /// A bad line in one bot's script, inside a mission.
    BotScript {
        bot: String,
        line: usize,
        text: String,
    },
    /// Two bots in one mission with the same name.
    DuplicateBot(String),
    /// A bot whose start is outside the map.
    OffMap { bot: String },
    /// A bot whose start is a wall.
    InWall { bot: String },
    /// A facing that isn't N, E, S or W.
    BadFacing { bot: String, facing: String },
    /// The mission file isn't valid JSON for a mission.
    Json(String),
    /// A file couldn't be read.
    Io(String),
    /// The command line didn't make sense.
    Usage(String),
}

impl fmt::Display for McError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            McError::EmptyMap => write!(f, "the map is empty"),
            McError::UnknownTile { line, col, ch } => {
                write!(f, "map line {line}, column {col}: unknown tile '{ch}'")
            }
            McError::RaggedMap { line } => {
                write!(
                    f,
                    "map line {line} is a different length from the first row"
                )
            }
            McError::BadCommand { line, text } => {
                write!(f, "script line {line}: can't understand '{text}'")
            }
            McError::BotScript { bot, line, text } => {
                write!(f, "{bot}'s script line {line}: can't understand '{text}'")
            }
            McError::DuplicateBot(name) => write!(f, "two bots are called {name}"),
            McError::OffMap { bot } => write!(f, "{bot} starts off the map"),
            McError::InWall { bot } => write!(f, "{bot} starts inside a wall"),
            McError::BadFacing { bot, facing } => {
                write!(f, "{bot} has an unknown facing '{facing}'")
            }
            McError::Json(msg) => write!(f, "can't read mission: {msg}"),
            McError::Io(msg) => write!(f, "can't read file: {msg}"),
            McError::Usage(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for McError {}
