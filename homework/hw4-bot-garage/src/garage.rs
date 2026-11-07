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
        let _ = f; // you will write! into f
        todo!("one message per variant, for {self:?}")
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
        todo!("{}", self.bots.len())
    }

    /// Watts still free: the budget minus the total power.
    pub fn available(&self) -> u32 {
        todo!("{}", self.budget)
    }

    /// Park a bot.
    /// - Names are compared ignoring case: "zippy" and "Zippy" are the same bot.
    ///   A name that's already parked gives `Duplicate`, with the new bot's name.
    /// - A bot that needs more than `available()` watts gives `OverBudget`.
    ///   Using exactly the watts left is fine.
    /// - The name check comes first.
    /// - A refused bot changes nothing.
    pub fn park(&mut self, bot: Box<dyn Bot>) -> Result<(), GarageError> {
        todo!("park {}", bot.name())
    }

    /// Take a bot out of the garage by name (ignoring case) and hand it back.
    /// The other bots keep their order. No such bot gives `NotFound` with the name asked for.
    pub fn unpark(&mut self, name: &str) -> Result<Box<dyn Bot>, GarageError> {
        todo!("unpark {name}")
    }

    /// The names of the parked bots, in alphabetical order.
    pub fn names(&self) -> Vec<&str> {
        todo!()
    }

    /// How many bots of each kind are parked, like {"drone": 2, "vacuum": 1}.
    pub fn count_by_kind(&self) -> HashMap<&'static str, usize> {
        let _counts: HashMap<&'static str, usize> = HashMap::new();
        todo!("use the entry API, as in the lab")
    }

    /// Labels of the bots drawing at least `watts`, in the order they were parked.
    pub fn power_hogs(&self, watts: u32) -> Vec<String> {
        todo!("bots drawing at least {watts} W")
    }
}
