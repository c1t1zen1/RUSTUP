//! Homework 5: Swarm Sim.
//!
//! A config file describes a dirty floor and a few cleaning bots. You read it
//! with serde, check it, run every bot on its own thread, and write a report
//! back out as JSON.
pub mod config;
pub mod sim;

use config::{Config, SimError};

/// The whole pipeline: JSON config in, JSON report out.
pub fn simulate_json(text: &str) -> Result<String, SimError> {
    let config = Config::from_json(text)?;
    Ok(sim::run(&config).to_json())
}
