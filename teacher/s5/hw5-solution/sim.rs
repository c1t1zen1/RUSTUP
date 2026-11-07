//! Running the swarm: one thread per bot, one shared floor.
use crate::config::{BotPlan, Config};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::thread;

/// What one bot did. Every field comes out the same on every run.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BotReport {
    pub name: String,
    /// Where it stopped, `[x, y]`.
    pub end: (usize, usize),
    /// Moves that went somewhere.
    pub moves: u32,
    /// Moves that hit the edge of the floor, so the bot stayed put.
    pub bumps: u32,
}

/// What the whole swarm did.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Report {
    /// One entry per bot, sorted by name.
    pub bots: Vec<BotReport>,
    /// Units of dirt removed, all bots together.
    pub cleaned: u32,
    /// Units of dirt still on the floor.
    pub dirt_left: u32,
    /// The floor at the end, same shape as the config's.
    pub floor: Vec<Vec<u32>>,
}

impl Report {
    /// The report as pretty-printed JSON (`serde_json::to_string_pretty`).
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("a Report always serializes")
    }
}

/// Take one unit of dirt from a tile, if it has any. Returns whether it did.
fn clean(floor: &Mutex<Vec<Vec<u32>>>, (x, y): (usize, usize)) -> bool {
    let mut floor = floor.lock().unwrap();
    let tile = &mut floor[y][x];
    if *tile > 0 {
        *tile -= 1;
        true
    } else {
        false
    }
}

/// One bot's whole shift. Runs on its own thread.
fn walk(plan: BotPlan, floor: Arc<Mutex<Vec<Vec<u32>>>>, width: usize, height: usize) -> BotReport {
    let (mut x, mut y) = plan.start;
    let mut moves = 0;
    let mut bumps = 0;
    clean(&floor, (x, y));
    for step in plan.route.chars() {
        let next = match step {
            'N' if y > 0 => Some((x, y - 1)),
            'S' if y + 1 < height => Some((x, y + 1)),
            'W' if x > 0 => Some((x - 1, y)),
            'E' if x + 1 < width => Some((x + 1, y)),
            _ => None,
        };
        match next {
            Some(tile) => {
                (x, y) = tile;
                moves += 1;
                clean(&floor, tile);
            }
            None => bumps += 1,
        }
    }
    BotReport {
        name: plan.name,
        end: (x, y),
        moves,
        bumps,
    }
}

/// Run the swarm. Each bot gets its own thread, and they all share one floor
/// behind an `Arc<Mutex<..>>`.
///
/// The rules for each bot:
/// - it cleans one unit from its start tile, if there's any dirt there
/// - for each letter of its route it tries to move one tile
/// - a move that would leave the floor is a bump: the bot stays put and cleans nothing
/// - a move that works cleans one unit from the new tile, if there's any dirt there
///
/// Which bot gets the last unit on a tile two bots visit depends on which
/// thread locks first, so the report doesn't say who cleaned what. The totals
/// and the final floor come out the same every time.
pub fn run(config: &Config) -> Report {
    let floor = Arc::new(Mutex::new(config.floor.clone()));
    let (width, height) = (config.width(), config.height());
    let handles: Vec<_> = config
        .bots
        .iter()
        .cloned()
        .map(|plan| {
            let floor = Arc::clone(&floor);
            thread::spawn(move || walk(plan, floor, width, height))
        })
        .collect();
    let mut bots: Vec<BotReport> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    bots.sort_by(|a, b| a.name.cmp(&b.name));

    let floor = floor.lock().unwrap().clone();
    let total = |f: &[Vec<u32>]| f.iter().flatten().sum::<u32>();
    let dirt_left = total(&floor);
    Report {
        bots,
        cleaned: total(&config.floor) - dirt_left,
        dirt_left,
        floor,
    }
}
