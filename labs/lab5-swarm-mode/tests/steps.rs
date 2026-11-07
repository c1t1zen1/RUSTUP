//! Lab 5 checks, one group per step. Run one step with: cargo test step3
use swarm_mode::{Job, count_squeaks, dust_in, run_jobs, scan_rooms, scoped_sum, total_dust};

fn rooms() -> Vec<String> {
    ["studio", "workshop", "lobby", "green room", "roof"]
        .iter()
        .map(|r| r.to_string())
        .collect()
}

#[test]
fn step1_readings_come_back_in_order() {
    let expected: Vec<u32> = rooms().iter().map(|r| dust_in(r)).collect();
    assert_eq!(scan_rooms(rooms()), expected);
}

#[test]
fn step1_no_rooms_no_threads() {
    assert_eq!(scan_rooms(Vec::new()), Vec::<u32>::new());
}

#[test]
fn step2_channel_total_matches() {
    let expected: u32 = rooms().iter().map(|r| dust_in(r)).sum();
    assert_eq!(total_dust(rooms()), expected);
    assert_eq!(total_dust(Vec::new()), 0);
}

#[test]
fn step3_every_squeak_counted() {
    assert_eq!(count_squeaks(5, 10), 25);
    assert_eq!(count_squeaks(8, 1_001), 4_000);
}

#[test]
fn step3_no_bots_no_squeaks() {
    assert_eq!(count_squeaks(0, 10), 0);
}

#[test]
fn step4_scoped_sum_any_split() {
    let readings: Vec<u32> = (1..=100).collect();
    for parts in [1, 3, 7, 100, 250] {
        assert_eq!(scoped_sum(&readings, parts), 5_050, "parts = {parts}");
    }
}

#[test]
fn step4_edge_cases() {
    assert_eq!(scoped_sum(&[], 4), 0);
    assert_eq!(scoped_sum(&[7, 8], 0), 15);
}

#[test]
fn step5_every_job_reported_in_id_order() {
    let jobs: Vec<Job> = rooms()
        .into_iter()
        .enumerate()
        .map(|(i, room)| Job {
            id: 5 - i as u32,
            room,
        })
        .collect();
    let reports = run_jobs(jobs, 3);
    let ids: Vec<u32> = reports.iter().map(|(id, _)| *id).collect();
    assert_eq!(ids, [1, 2, 3, 4, 5]);
    assert_eq!(reports[0].1, format!("roof: {}", dust_in("roof")));
}

#[test]
fn step5_worker_count_does_not_change_the_answer() {
    let jobs: Vec<Job> = (0..40)
        .map(|i| Job {
            id: i,
            room: format!("bay {i}"),
        })
        .collect();
    let one = run_jobs(jobs.clone(), 1);
    assert_eq!(one.len(), 40);
    assert_eq!(run_jobs(jobs.clone(), 6), one);
    assert_eq!(run_jobs(jobs, 0), one);
}
