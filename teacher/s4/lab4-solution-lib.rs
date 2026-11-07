//! Lab 4: Robbie's Mission Planner.
//!
//! Robbie has a to-do list for the studio and only so much battery. You'll use
//! iterators, closures and a HashMap to turn the list into a plan.
use std::collections::HashMap;

/// One job on Robbie's list.
#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub name: String,
    pub room: String,
    pub minutes: u32,
    /// 1 (whenever) to 5 (right now)
    pub priority: u8,
    pub done: bool,
}

impl Task {
    /// A shortcut for building tasks in tests and in main.
    pub fn new(name: &str, room: &str, minutes: u32, priority: u8) -> Task {
        Task {
            name: name.to_string(),
            room: room.to_string(),
            minutes,
            priority,
            done: false,
        }
    }
}

/// Step 1. The total minutes of every task that isn't done yet.
pub fn minutes_left(tasks: &[Task]) -> u32 {
    tasks.iter().filter(|t| !t.done).map(|t| t.minutes).sum()
}

/// Step 2. The tasks that aren't done, most urgent first.
/// Higher priority comes first; for equal priority, shorter tasks first;
/// for equal priority and minutes, alphabetical by name.
pub fn todo_list(tasks: &[Task]) -> Vec<&Task> {
    let mut list: Vec<&Task> = tasks.iter().filter(|t| !t.done).collect();
    list.sort_by(|a, b| {
        b.priority
            .cmp(&a.priority)
            .then(a.minutes.cmp(&b.minutes))
            .then(a.name.cmp(&b.name))
    });
    list
}

/// Step 3. Minutes still to do in each room. Rooms with nothing left to do
/// don't appear at all.
pub fn minutes_by_room(tasks: &[Task]) -> HashMap<String, u32> {
    let mut rooms = HashMap::new();
    for t in tasks.iter().filter(|t| !t.done) {
        *rooms.entry(t.room.clone()).or_insert(0) += t.minutes;
    }
    rooms
}

/// Step 4. Plan a shift: walk the todo_list in order and take every task that
/// still fits in the minutes left. A task that doesn't fit is skipped, and the
/// planner keeps looking for shorter ones. Returns the names in the order taken.
pub fn plan(tasks: &[Task], budget: u32) -> Vec<&str> {
    let mut left = budget;
    let mut names = Vec::new();
    for t in todo_list(tasks) {
        if t.minutes <= left {
            left -= t.minutes;
            names.push(t.name.as_str());
        }
    }
    names
}

/// Step 5. The first task in `room` that isn't done, if there is one.
/// The answer borrows from `tasks`, not from `room`, and the lifetime says so.
pub fn next_in_room<'a>(tasks: &'a [Task], room: &str) -> Option<&'a Task> {
    tasks.iter().find(|t| !t.done && t.room == room)
}
