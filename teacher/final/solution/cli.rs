//! The command line, kept in the library so it can be tested.

use crate::{BotSpec, McError, Mission, Tile, World, parse_script, run_fleet, run_mission};

/// How to call the program.
pub const USAGE: &str = "usage:
  robbie-mc MISSION.json [--fleet] [--json]
  robbie-mc MAP.txt SCRIPT.txt [--json]";

fn read(path: &str) -> Result<String, McError> {
    std::fs::read_to_string(path).map_err(|e| McError::Io(format!("{path}: {e}")))
}

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
    let json = args.iter().any(|a| a == "--json");
    let fleet = args.iter().any(|a| a == "--fleet");
    if let Some(flag) = args
        .iter()
        .find(|a| a.starts_with("--") && *a != "--json" && *a != "--fleet")
    {
        return Err(McError::Usage(format!("unknown option {flag}\n{USAGE}")));
    }
    let files: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();
    let report = match files.as_slice() {
        [mission] => {
            let mission = Mission::from_json(&read(mission)?)?;
            if fleet {
                run_fleet(&mission)?
            } else {
                run_mission(&mission)?
            }
        }
        [map, script] if !fleet => {
            let map_text = read(map)?;
            let world = World::parse(&map_text)?;
            let script_text = read(script)?;
            parse_script(&script_text)?;
            let start = (0..world.height())
                .flat_map(|y| (0..world.width()).map(move |x| (x, y)))
                .find(|&(x, y)| world.tile(x, y) != Some(Tile::Wall))
                .ok_or_else(|| McError::Usage("the map has nowhere to stand".to_string()))?;
            let mission = Mission {
                map: map_text,
                bots: vec![BotSpec {
                    name: "Robbie".to_string(),
                    start,
                    facing: "E".to_string(),
                    battery: 50,
                    script: script_text,
                }],
            };
            run_mission(&mission)?
        }
        _ => return Err(McError::Usage(USAGE.to_string())),
    };
    Ok(if json {
        report.to_json()
    } else {
        report.to_text()
    })
}
