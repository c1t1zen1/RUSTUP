//! Homework 4: Bot Garage.
pub mod bots;
pub mod garage;

use bots::Bot;

/// The largest item in a slice, or None when it's empty.
/// Works for any type that can be compared and copied: numbers, chars, bools.
pub fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    let mut best = *items.first()?;
    for &item in items {
        if item > best {
            best = item;
        }
    }
    Some(best)
}

/// The bot that draws the most power from a slice of one kind of bot.
/// When two draw the same, the first one wins. None for an empty slice.
pub fn hungriest<B: Bot>(bots: &[B]) -> Option<&B> {
    let mut best = bots.first()?;
    for b in bots {
        if b.power() > best.power() {
            best = b;
        }
    }
    Some(best)
}
