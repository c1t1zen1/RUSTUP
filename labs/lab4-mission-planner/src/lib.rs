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
    todo!(
        "step 1: add up the minutes of {} tasks, skipping done ones",
        tasks.len()
    )
}

/// Step 2. The tasks that aren't done, most urgent first.
/// Higher priority comes first; for equal priority, shorter tasks first;
/// for equal priority and minutes, alphabetical by name.
pub fn todo_list(tasks: &[Task]) -> Vec<&Task> {
    todo!(
        "step 2: filter, collect, then sort_by ({} tasks)",
        tasks.len()
    )
}

/// Step 3. Minutes still to do in each room. Rooms with nothing left to do
/// don't appear at all.
pub fn minutes_by_room(tasks: &[Task]) -> HashMap<String, u32> {
    let _rooms: HashMap<String, u32> = HashMap::new();
    todo!(
        "step 3: use rooms.entry(..).or_insert(0) ({} tasks)",
        tasks.len()
    )
}

/// Step 4. Plan a shift: walk the todo_list in order and take every task that
/// still fits in the minutes left. A task that doesn't fit is skipped, and the
/// planner keeps looking for shorter ones. Returns the names in the order taken.
pub fn plan(tasks: &[Task], budget: u32) -> Vec<&str> {
    todo!("step 4: fit {} tasks into {budget} minutes", tasks.len())
}

/// Step 5. The first task in `room` that isn't done, if there is one.
/// The answer borrows from `tasks`, not from `room`, and the lifetime says so.
pub fn next_in_room<'a>(tasks: &'a [Task], room: &str) -> Option<&'a Task> {
    todo!(
        "step 5: find the first open task in {room} ({} tasks)",
        tasks.len()
    )
}
