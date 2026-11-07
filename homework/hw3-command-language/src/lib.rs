//! Homework 3: Robbie's command language.
//! A script is one command per line. Blank lines and lines starting with # are skipped.
pub mod parse;
pub mod robot;

use parse::ParseError; // you will want parse::parse_line too
use robot::{Robot, RobotError};
use std::fmt;

/// What went wrong, and on which line (counting from 1, blank lines included).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptError {
    Parse { line: usize, error: ParseError },
    Robot { line: usize, error: RobotError },
}

impl fmt::Display for ScriptError {
    /// "line 3: unknown command 'jump'", using the inner error's own message
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("match on {self:?} and write! the message into {f:p}")
    }
}

/// Run a whole script on a fresh Robbie. Returns where he ended up and everything he said,
/// or the first error, which stops the script.
///
/// Rules: lines are numbered from 1, counting blank lines and comments. Skip a line that is blank
/// or starts with # (after trimming). Collect every Some(..) that apply returns, in order.
/// Hint: .lines().enumerate(), then parse_line(..).map_err(..)? and robbie.apply(..).map_err(..)?
pub fn run_script(script: &str) -> Result<(Robot, Vec<String>), ScriptError> {
    todo!("run {script:?} on Robot::new()")
}
