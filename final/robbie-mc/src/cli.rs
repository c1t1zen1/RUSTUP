//! The command line, kept in the library so it can be tested.

use crate::McError;

/// How to call the program.
pub const USAGE: &str = "usage:
  robbie-mc MISSION.json [--fleet] [--json]
  robbie-mc MAP.txt SCRIPT.txt [--json]";

/// Runs the program on its arguments (not counting the program name) and
/// returns what it should print.
///
/// - `MISSION.json` runs a mission file one bot at a time.
///   Add `--fleet` to run every bot on its own thread.
/// - `MAP.txt SCRIPT.txt` runs one bot called Robbie, starting on the first
///   tile that isn't a wall (reading left to right, top to bottom), facing
///   East with a battery of 50.
/// - `--json` prints the report as JSON instead of a table.
///
/// Anything else is `McError::Usage` with [`USAGE`] as its message.
pub fn run_cli(args: &[String]) -> Result<String, McError> {
    todo!("run the program on {args:?}")
}
