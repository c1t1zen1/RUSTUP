//! # Robbie Mission Control
//!
//! Load a map and a crew of Robbies, give each one a script, and run the
//! mission, one bot at a time or the whole fleet at once on threads.
//!
//! ```
//! use robbie_mc::{Mission, run_mission};
//! let mission = Mission::from_json(r#"{
//!     "map": "....\n..P.",
//!     "bots": [{ "name": "Robbie", "start": [0, 1], "facing": "E",
//!                "battery": 10, "script": "forward 2\npick" }]
//! }"#).unwrap();
//! let report = run_mission(&mission).unwrap();
//! assert_eq!(report.bots[0].end, (2, 1));
//! assert_eq!(report.parts_collected, 1);
//! ```

mod bot;
mod cli;
mod command;
mod error;
mod mission;
mod world;

pub use bot::{Bot, Facing};
pub use cli::{USAGE, run_cli};
pub use command::{Command, parse_script};
pub use error::McError;
pub use mission::{BotReport, BotSpec, Crew, Mission, Report, run_fleet, run_mission};
pub use world::{Tile, World};
