//! Lab 5: Swarm Mode. Teacher's solution.
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

/// How much dust is in a room. Pretend it takes a scanner a while.
/// Given to you: it's the same answer every time for the same room.
pub fn dust_in(room: &str) -> u32 {
    room.bytes().map(u32::from).sum::<u32>() % 50 + room.len() as u32
}

/// Step 1. One thread per room, each running `dust_in`. Returns the readings
/// in the same order as `rooms`, whichever thread finishes first.
pub fn scan_rooms(rooms: Vec<String>) -> Vec<u32> {
    let handles: Vec<_> = rooms
        .into_iter()
        .map(|room| thread::spawn(move || dust_in(&room)))
        .collect();
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

/// Step 2. One thread per room again, but each one sends its reading down a
/// channel, and the caller adds up whatever arrives.
pub fn total_dust(rooms: Vec<String>) -> u32 {
    let (tx, rx) = mpsc::channel();
    for room in rooms {
        let tx = tx.clone();
        thread::spawn(move || tx.send(dust_in(&room)).unwrap());
    }
    drop(tx);
    rx.iter().sum()
}

/// Step 3. `bots` threads share one squeak counter. Each bot takes
/// `steps_each` steps and squeaks on every second step (steps 2, 4, 6...).
/// Returns the final count.
pub fn count_squeaks(bots: usize, steps_each: u32) -> u32 {
    let squeaks = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..bots {
        let squeaks = Arc::clone(&squeaks);
        handles.push(thread::spawn(move || {
            for step in 1..=steps_each {
                if step % 2 == 0 {
                    *squeaks.lock().unwrap() += 1;
                }
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    *squeaks.lock().unwrap()
}

/// Step 4. Split `readings` into `parts` chunks, add each chunk on its own
/// scoped thread, and return the grand total. `parts` of 0 counts as 1.
pub fn scoped_sum(readings: &[u32], parts: usize) -> u32 {
    let size = readings.len().div_ceil(parts.max(1)).max(1);
    thread::scope(|s| {
        let handles: Vec<_> = readings
            .chunks(size)
            .map(|chunk| s.spawn(move || chunk.iter().sum::<u32>()))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    })
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
    let (job_tx, job_rx) = mpsc::channel::<Job>();
    let job_rx = Arc::new(Mutex::new(job_rx));
    let (done_tx, done_rx) = mpsc::channel();

    let mut handles = Vec::new();
    for _ in 0..workers.max(1) {
        let job_rx = Arc::clone(&job_rx);
        let done_tx = done_tx.clone();
        handles.push(thread::spawn(move || {
            loop {
                let next = job_rx.lock().unwrap().recv();
                let Ok(job) = next else { break };
                let report = format!("{}: {}", job.room, dust_in(&job.room));
                done_tx.send((job.id, report)).unwrap();
            }
        }));
    }
    drop(done_tx);

    for job in jobs {
        job_tx.send(job).unwrap();
    }
    drop(job_tx);

    let mut reports: Vec<(u32, String)> = done_rx.iter().collect();
    for h in handles {
        h.join().unwrap();
    }
    reports.sort();
    reports
}
