use swarm_mode::{Job, count_squeaks, dust_in, run_jobs, scan_rooms, scoped_sum, total_dust};

fn main() {
    let rooms: Vec<String> = ["studio", "workshop", "lobby", "green room", "roof"]
        .iter()
        .map(|r| r.to_string())
        .collect();

    println!("Swarm mode: five Robbies, five rooms.\n");
    let readings = scan_rooms(rooms.clone());
    for (room, dust) in rooms.iter().zip(&readings) {
        println!("  {room:<11} {dust:>3} dust");
    }
    println!("\nTotal by channel: {}", total_dust(rooms.clone()));
    println!("Total by scoped threads: {}", scoped_sum(&readings, 2));
    println!(
        "Squeaks from 5 bots walking 10 steps: {}",
        count_squeaks(5, 10)
    );

    let jobs: Vec<Job> = rooms
        .iter()
        .enumerate()
        .map(|(i, r)| Job {
            id: i as u32 + 1,
            room: r.clone(),
        })
        .collect();
    println!("\nWorker pool, 3 workers:");
    for (id, report) in run_jobs(jobs, 3) {
        println!("  job {id}: {report}");
    }
    println!(
        "\n(The roof alone: {} dust. Robbie doesn't go up there. Rain.)",
        dust_in("roof")
    );
}
