//! Lab 4 checks, one group per step. Run one step with: cargo test step3
use mission_planner::{Task, minutes_by_room, minutes_left, next_in_room, plan, todo_list};

fn studio() -> Vec<Task> {
    let mut oil = Task::new("oil knees", "workshop", 20, 5);
    oil.done = true;
    vec![
        Task::new("dust the LED wall", "studio", 30, 2),
        Task::new("check the cameras", "studio", 15, 4),
        oil,
        Task::new("sort bolts", "workshop", 45, 1),
        Task::new("find umbrella", "lobby", 10, 5),
        Task::new("charge batteries", "workshop", 25, 4),
    ]
}

#[test]
fn step1_minutes_left() {
    assert_eq!(minutes_left(&studio()), 125);
    assert_eq!(minutes_left(&[]), 0);
}

#[test]
fn step2_todo_list_order() {
    let tasks = studio();
    let names: Vec<&str> = todo_list(&tasks).iter().map(|t| t.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "find umbrella",
            "check the cameras",
            "charge batteries",
            "dust the LED wall",
            "sort bolts"
        ]
    );
}

#[test]
fn step2_ties_break_alphabetically() {
    let tasks = vec![Task::new("b", "x", 5, 3), Task::new("a", "x", 5, 3)];
    let names: Vec<&str> = todo_list(&tasks).iter().map(|t| t.name.as_str()).collect();
    assert_eq!(names, ["a", "b"]);
}

#[test]
fn step3_minutes_by_room() {
    let rooms = minutes_by_room(&studio());
    assert_eq!(rooms.len(), 3);
    assert_eq!(rooms["studio"], 45);
    assert_eq!(rooms["workshop"], 70);
    assert_eq!(rooms["lobby"], 10);
}

#[test]
fn step3_done_rooms_disappear() {
    let mut t = Task::new("nap", "attic", 60, 1);
    t.done = true;
    assert!(minutes_by_room(&[t]).is_empty());
}

#[test]
fn step4_plan_fills_the_budget() {
    let tasks = studio();
    assert_eq!(
        plan(&tasks, 60),
        ["find umbrella", "check the cameras", "charge batteries"]
    );
}

#[test]
fn step4_plan_skips_what_does_not_fit() {
    let tasks = studio();
    // 10 + 15 = 25, charge (25) would make 50 > 40, dust (30) too big, sort (45) too big
    assert_eq!(plan(&tasks, 40), ["find umbrella", "check the cameras"]);
    assert!(plan(&tasks, 5).is_empty());
}

#[test]
fn step5_next_in_room() {
    let tasks = studio();
    let next = next_in_room(&tasks, "workshop").map(|t| t.name.as_str());
    assert_eq!(next, Some("sort bolts"));
    assert_eq!(next_in_room(&tasks, "roof"), None);
}

#[test]
fn step5_answer_outlives_the_room_name() {
    let tasks = studio();
    let found;
    {
        let room = String::from("lobby");
        found = next_in_room(&tasks, &room);
    } // room is gone here, but found only borrows tasks
    assert_eq!(found.map(|t| t.minutes), Some(10));
}
