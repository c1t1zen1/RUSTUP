//! Homework 3: Robbie's command language.
//! A script is one command per line. Blank lines and lines starting with # are skipped.
pub mod parse;
pub mod robot;

use parse::{ParseError, parse_line};
use robot::{Robot, RobotError};
use std::fmt;

/// What went wrong, and on which line (counting from 1, blank lines included).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptError {
    Parse { line: usize, error: ParseError },
    Robot { line: usize, error: RobotError },
}

impl fmt::Display for ScriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScriptError::Parse { line, error } => write!(f, "line {line}: {error}"),
            ScriptError::Robot { line, error } => write!(f, "line {line}: {error}"),
        }
    }
}

/// Run a whole script on a fresh Robbie. Returns where he ended up and everything he said,
/// or the first error, which stops the script.
pub fn run_script(script: &str) -> Result<(Robot, Vec<String>), ScriptError> {
    let mut robbie = Robot::new();
    let mut said = Vec::new();
    for (i, text) in script.lines().enumerate() {
        let line = i + 1;
        let trimmed = text.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let cmd = parse_line(trimmed).map_err(|error| ScriptError::Parse { line, error })?;
        let out = robbie
            .apply(&cmd)
            .map_err(|error| ScriptError::Robot { line, error })?;
        if let Some(out) = out {
            said.push(out);
        }
    }
    Ok((robbie, said))
}
