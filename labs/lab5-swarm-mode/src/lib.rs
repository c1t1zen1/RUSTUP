//! Lab 5: Swarm Mode.
//!
//! Five Robbies, five rooms, one lab. You'll spread the work across threads,
//! pass results down channels, and share a counter safely.
//!
//! Add the imports as you need them. Between them, the five steps use:
//! `std::thread`, `std::sync::mpsc`, and `std::sync::{Arc, Mutex}`.

/// How much dust is in a room. Pretend it takes a scanner a while.
/// Given to you: it's the same answer every time for the same room.
pub fn dust_in(room: &str) -> u32 {
    room.bytes().map(u32::from).sum::<u32>() % 50 + room.len() as u32
}

/// Step 1. One thread per room, each running `dust_in`. Returns the readings
/// in the same order as `rooms`, whichever thread finishes first.
pub fn scan_rooms(rooms: Vec<String>) -> Vec<u32> {
    todo!(
        "step 1: spawn a thread per room ({} rooms), then join in order",
        rooms.len()
    )
}

/// Step 2. One thread per room again, but each one sends its reading down a
/// channel, and the caller adds up whatever arrives.
pub fn total_dust(rooms: Vec<String>) -> u32 {
    todo!(
        "step 2: send each reading down a channel ({} rooms)",
        rooms.len()
    )
}

/// Step 3. `bots` threads share one squeak counter. Each bot takes
/// `steps_each` steps and squeaks on every second step (steps 2, 4, 6...).
/// Returns the final count.
pub fn count_squeaks(bots: usize, steps_each: u32) -> u32 {
    todo!("step 3: {bots} bots, {steps_each} steps each, one Arc<Mutex<u32>>")
}

/// Step 4. Split `readings` into `parts` chunks, add each chunk on its own
/// scoped thread, and return the grand total. `parts` of 0 counts as 1.
pub fn scoped_sum(readings: &[u32], parts: usize) -> u32 {
    todo!(
        "step 4: add {} readings in {parts} chunks with thread::scope",
        readings.len()
    )
}

/// One cleaning job for the swarm.
#[derive(Debug, Clone, PartialEq)]
pub struct Job {
    pub id: u32,
    pub room: String,
}

/// Step 5. A worker pool. Start `workers` threads (0 counts as 1) that share
/// one job queue. Each worker takes jobs until the queue is empty and reports
/// `(id, "room: dust")` for each. Returns every report, sorted by id.
pub fn run_jobs(jobs: Vec<Job>, workers: usize) -> Vec<(u32, String)> {
    todo!(
        "step 5: {} jobs, {workers} workers sharing one queue",
        jobs.len()
    )
}
