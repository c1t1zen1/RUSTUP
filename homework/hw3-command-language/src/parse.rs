//! Turning one line of Robbie's command language into a Command.
use std::fmt;

/// Everything Robbie understands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Forward(u32),
    Back(u32),
    Left,
    Right,
    Say(String),
    Charge,
    Report,
}

/// Why a line couldn't be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The line had nothing on it.
    Empty,
    /// The first word isn't a command Robbie knows.
    UnknownCommand(String),
    /// The command needs something after it: "distance" or "text".
    MissingArgument(&'static str),
    /// The distance wasn't a whole number of zero or more.
    BadNumber(String),
}

impl fmt::Display for ParseError {
    /// "empty line", "unknown command 'dance'", "missing distance", "'lots' is not a whole number"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("match on {self:?} and write! the message into {f:p}")
    }
}

/// Read one line. Command words are case-insensitive; the text after "say" keeps its case.
/// Rules:
/// - Trim the line. Nothing left is ParseError::Empty.
/// - The first word picks the command, in any case: forward, back, left, right, say, charge, report.
///   Anything else is UnknownCommand(word), with the word as typed.
/// - forward and back need a distance: nothing after them is MissingArgument("distance"),
///   and anything that isn't a u32 is BadNumber(the rest of the line, trimmed).
/// - say needs text: nothing after it is MissingArgument("text"). The text keeps its case.
///
/// Hint: write a small helper fn for the distance and call it with ?
pub fn parse_line(line: &str) -> Result<Command, ParseError> {
    todo!("parse {line:?}")
}
