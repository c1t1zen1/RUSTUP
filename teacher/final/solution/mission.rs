//! Missions: a map, some bots, a script each, and the report at the end.

use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;

use serde::{Deserialize, Serialize};

use crate::{Bot, Command, Facing, McError, Tile, World, parse_script};

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
        serde_json::from_str(text).map_err(|e| McError::Json(e.to_string()))
    }

    /// Checks everything before anything moves, and builds the world and
    /// the bots.
    ///
    /// The map is checked first. Then each bot in order: a repeated name,
    /// a start off the map, a start inside a wall, the facing, then its
    /// script. The first problem found is the one returned.
    pub fn prepare(&self) -> Result<(World, Crew), McError> {
        let world = World::parse(&self.map)?;
        let mut seen = HashSet::new();
        let mut crew = Vec::new();
        for spec in &self.bots {
            if !seen.insert(spec.name.as_str()) {
                return Err(McError::DuplicateBot(spec.name.clone()));
            }
            let (x, y) = spec.start;
            match world.tile(x, y) {
                None => {
                    return Err(McError::OffMap {
                        bot: spec.name.clone(),
                    });
                }
                Some(Tile::Wall) => {
                    return Err(McError::InWall {
                        bot: spec.name.clone(),
                    });
                }
                Some(_) => {}
            }
            let facing = Facing::parse(&spec.facing).ok_or_else(|| McError::BadFacing {
                bot: spec.name.clone(),
                facing: spec.facing.clone(),
            })?;
            let cmds = parse_script(&spec.script).map_err(|e| match e {
                McError::BadCommand { line, text } => McError::BotScript {
                    bot: spec.name.clone(),
                    line,
                    text,
                },
                other => other,
            })?;
            crew.push((Bot::new(&spec.name, x, y, facing, spec.battery), cmds));
        }
        Ok((world, crew))
    }
}

fn report(world: &World, bots: &[Bot]) -> Report {
    Report {
        bots: bots
            .iter()
            .map(|b| BotReport {
                name: b.name.clone(),
                end: (b.x, b.y),
                facing: b.facing.letter(),
                battery: b.battery,
                parts: b.parts,
                steps: b.steps,
                bumps: b.bumps,
                flat: b.flat,
            })
            .collect(),
        parts_collected: bots.iter().map(|b| b.parts).sum(),
        parts_left: world.parts_left(),
        map: world.render(),
    }
}

/// Runs a mission one bot at a time, in order: the first bot runs its whole
/// script, then the second starts on the world the first one left.
pub fn run_mission(mission: &Mission) -> Result<Report, McError> {
    let (mut world, crew) = mission.prepare()?;
    let mut bots = Vec::new();
    for (mut bot, cmds) in crew {
        for cmd in cmds {
            bot.run(&mut world, cmd);
        }
        bots.push(bot);
    }
    Ok(report(&world, &bots))
}

/// Runs a mission with every bot on its own thread, all at once, sharing
/// one world.
///
/// Bots never block each other, so every bot ends in the same place as in
/// `run_mission`. When two bots reach the same part, which one gets it can
/// change from run to run, but the totals never do.
pub fn run_fleet(mission: &Mission) -> Result<Report, McError> {
    let (world, crew) = mission.prepare()?;
    let world = Arc::new(Mutex::new(world));
    let handles: Vec<_> = crew
        .into_iter()
        .map(|(mut bot, cmds)| {
            let world = Arc::clone(&world);
            thread::spawn(move || {
                for cmd in cmds {
                    // Lock for one command at a time, so the bots take turns.
                    let mut w = world.lock().unwrap_or_else(|e| e.into_inner());
                    bot.run(&mut w, cmd);
                }
                bot
            })
        })
        .collect();
    let mut bots = Vec::new();
    for h in handles {
        match h.join() {
            Ok(bot) => bots.push(bot),
            Err(_) => return Err(McError::Usage("a bot's thread crashed".to_string())),
        }
    }
    let world = world.lock().unwrap_or_else(|e| e.into_inner());
    Ok(report(&world, &bots))
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
        let mut out = String::from("Mission report\n");
        for b in &self.bots {
            out.push_str(&format!(
                "  {:<8} at ({}, {}) facing {}   battery {:<3} parts {:<3} steps {:<3} bumps {}{}\n",
                b.name,
                b.end.0,
                b.end.1,
                b.facing,
                b.battery,
                b.parts,
                b.steps,
                b.bumps,
                if b.flat { "   FLAT" } else { "" }
            ));
        }
        out.push_str(&format!(
            "Parts collected: {}. Parts left: {}.",
            self.parts_collected, self.parts_left
        ));
        out
    }
}
