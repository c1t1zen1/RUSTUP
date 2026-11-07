//! Robbie's command language.

use crate::McError;

/// One instruction for a bot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    /// Drive this many tiles straight ahead.
    Forward(u32),
    /// Turn a quarter turn to the left, without moving.
    Left,
    /// Turn a quarter turn to the right, without moving.
    Right,
    /// Pick up the part on this tile, if there is one.
    Pick,
    /// Do nothing for one command.
    Wait,
}

/// Reads a script, one command per line.
///
/// The commands are `forward`, `forward N` (N from 1 up), `left`, `right`,
/// `pick` and `wait`, in any mix of upper and lower case. Spaces around a
/// command are ignored, and so are blank lines and lines starting with `#`.
///
/// ```
/// use robbie_mc::{parse_script, Command};
/// let cmds = parse_script("# warm up\nforward 2\nLEFT\n\npick").unwrap();
/// assert_eq!(cmds, vec![Command::Forward(2), Command::Left, Command::Pick]);
/// ```
pub fn parse_script(src: &str) -> Result<Vec<Command>, McError> {
    let mut cmds = Vec::new();
    for (i, raw) in src.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let bad = || McError::BadCommand {
            line: i + 1,
            text: line.to_string(),
        };
        let lower = line.to_lowercase();
        let words: Vec<&str> = lower.split_whitespace().collect();
        let cmd = match words.as_slice() {
            ["forward"] => Command::Forward(1),
            ["forward", n] => match n.parse::<u32>() {
                Ok(n) if n > 0 => Command::Forward(n),
                _ => return Err(bad()),
            },
            ["left"] => Command::Left,
            ["right"] => Command::Right,
            ["pick"] => Command::Pick,
            ["wait"] => Command::Wait,
            _ => return Err(bad()),
        };
        cmds.push(cmd);
    }
    Ok(cmds)
}
