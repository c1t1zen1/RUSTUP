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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Empty => write!(f, "empty line"),
            ParseError::UnknownCommand(word) => write!(f, "unknown command '{word}'"),
            ParseError::MissingArgument(what) => write!(f, "missing {what}"),
            ParseError::BadNumber(text) => write!(f, "'{text}' is not a whole number"),
        }
    }
}

/// Read one line. Command words are case-insensitive; the text after "say" keeps its case.
pub fn parse_line(line: &str) -> Result<Command, ParseError> {
    let line = line.trim();
    if line.is_empty() {
        return Err(ParseError::Empty);
    }
    let (word, rest) = match line.split_once(char::is_whitespace) {
        Some((w, r)) => (w, r.trim()),
        None => (line, ""),
    };
    match word.to_lowercase().as_str() {
        "forward" => Ok(Command::Forward(distance(rest)?)),
        "back" => Ok(Command::Back(distance(rest)?)),
        "left" => Ok(Command::Left),
        "right" => Ok(Command::Right),
        "say" if rest.is_empty() => Err(ParseError::MissingArgument("text")),
        "say" => Ok(Command::Say(rest.to_string())),
        "charge" => Ok(Command::Charge),
        "report" => Ok(Command::Report),
        _ => Err(ParseError::UnknownCommand(word.to_string())),
    }
}

fn distance(rest: &str) -> Result<u32, ParseError> {
    if rest.is_empty() {
        return Err(ParseError::MissingArgument("distance"));
    }
    rest.parse()
        .map_err(|_| ParseError::BadNumber(rest.to_string()))
}
