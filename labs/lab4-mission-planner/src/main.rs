use mission_planner::{Task, minutes_by_room, minutes_left, next_in_room, plan, todo_list};

fn main() {
    let mut tasks = vec![
        Task::new("dust the LED wall", "studio", 30, 2),
        Task::new("check the cameras", "studio", 15, 4),
        Task::new("oil knees", "workshop", 20, 5),
        Task::new("sort bolts", "workshop", 45, 1),
        Task::new("find umbrella", "lobby", 10, 5),
        Task::new("charge batteries", "workshop", 25, 4),
    ];
    tasks[2].done = true; // Robbie oiled his knees already. They still squeak.

    println!(
        "Robbie's to-do list ({} minutes left):",
        minutes_left(&tasks)
    );
    for t in todo_list(&tasks) {
        println!(
            "  [{}] {:<20} {:>3} min  ({})",
            t.priority, t.name, t.minutes, t.room
        );
    }

    let mut rooms: Vec<(String, u32)> = minutes_by_room(&tasks).into_iter().collect();
    rooms.sort();
    println!("\nBy room:");
    for (room, minutes) in rooms {
        println!("  {room:<10} {minutes:>3} min");
    }

    let budget = 60;
    println!("\nOne {budget}-minute shift: {:?}", plan(&tasks, budget));

    if let Some(t) = next_in_room(&tasks, "workshop") {
        println!("Next in the workshop: {}", t.name);
    }
}
