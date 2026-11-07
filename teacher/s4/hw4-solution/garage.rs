//! The garage: a power budget and a row of parked bots of any kind.
use crate::bots::Bot;
use std::collections::HashMap;
use std::fmt;

/// Everything that can go wrong at the garage door.
#[derive(Debug, Clone, PartialEq)]
pub enum GarageError {
    /// A bot with this name is already parked.
    Duplicate(String),
    /// The bot needs more power than the garage has left.
    OverBudget { needed: u32, available: u32 },
    /// No bot with this name is parked.
    NotFound(String),
}

/// Messages:
/// - Duplicate("Zippy")                    -> "Zippy is already parked"
/// - OverBudget { needed: 60, available: 20 } -> "needs 60 W, only 20 W left"
/// - NotFound("Zippy")                     -> "no bot called Zippy"
impl fmt::Display for GarageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GarageError::Duplicate(name) => write!(f, "{name} is already parked"),
            GarageError::OverBudget { needed, available } => {
                write!(f, "needs {needed} W, only {available} W left")
            }
            GarageError::NotFound(name) => write!(f, "no bot called {name}"),
        }
    }
}

/// A garage with a fixed power budget, in watts. It holds bots of different
/// types side by side, which is what `Box<dyn Bot>` is for.
pub struct Garage {
    bots: Vec<Box<dyn Bot>>,
    budget: u32,
}

impl Garage {
    /// An empty garage with `budget` watts to share out.
    pub fn new(budget: u32) -> Garage {
        Garage {
            bots: Vec::new(),
            budget,
        }
    }

    /// How many bots are parked.
    pub fn len(&self) -> usize {
        self.bots.len()
    }

    /// True when nothing is parked.
    pub fn is_empty(&self) -> bool {
        self.bots.is_empty()
    }

    /// Watts drawn by every parked bot together.
    pub fn total_power(&self) -> u32 {
        self.bots.iter().map(|b| b.power()).sum()
    }

    /// Watts still free: the budget minus the total power.
    pub fn available(&self) -> u32 {
        self.budget - self.total_power()
    }

    /// Park a bot.
    /// - Names are compared ignoring case: "zippy" and "Zippy" are the same bot.
    ///   A name that's already parked gives `Duplicate`, with the new bot's name.
    /// - A bot that needs more than `available()` watts gives `OverBudget`.
    ///   Using exactly the watts left is fine.
    /// - The name check comes first.
    /// - A refused bot changes nothing.
    pub fn park(&mut self, bot: Box<dyn Bot>) -> Result<(), GarageError> {
        if self.position(bot.name()).is_some() {
            return Err(GarageError::Duplicate(bot.name().to_string()));
        }
        let available = self.available();
        if bot.power() > available {
            return Err(GarageError::OverBudget {
                needed: bot.power(),
                available,
            });
        }
        self.bots.push(bot);
        Ok(())
    }

    /// Take a bot out of the garage by name (ignoring case) and hand it back.
    /// The other bots keep their order. No such bot gives `NotFound` with the name asked for.
    pub fn unpark(&mut self, name: &str) -> Result<Box<dyn Bot>, GarageError> {
        let i = self
            .position(name)
            .ok_or_else(|| GarageError::NotFound(name.to_string()))?;
        Ok(self.bots.remove(i))
    }

    /// The names of the parked bots, in alphabetical order.
    pub fn names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.bots.iter().map(|b| b.name()).collect();
        names.sort();
        names
    }

    /// How many bots of each kind are parked, like {"drone": 2, "vacuum": 1}.
    pub fn count_by_kind(&self) -> HashMap<&'static str, usize> {
        let mut counts = HashMap::new();
        for b in &self.bots {
            *counts.entry(b.kind()).or_insert(0) += 1;
        }
        counts
    }

    /// Labels of the bots drawing at least `watts`, in the order they were parked.
    pub fn power_hogs(&self, watts: u32) -> Vec<String> {
        self.bots
            .iter()
            .filter(|b| b.power() >= watts)
            .map(|b| b.label())
            .collect()
    }

    fn position(&self, name: &str) -> Option<usize> {
        self.bots
            .iter()
            .position(|b| b.name().eq_ignore_ascii_case(name))
    }
}
