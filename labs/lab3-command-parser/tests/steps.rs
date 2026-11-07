//! Lab 3 checks, one group per step. Run one step with: cargo test step3
use command_parser::{Command, Direction, Robot, parse};

#[test]
fn step1_new_robot() {
    let r = Robot::new();
    assert_eq!(
        (r.x, r.y, r.facing, r.battery),
        (0, 0, Direction::North, 100)
    );
}

#[test]
fn step2_turns() {
    assert_eq!(Direction::North.turn_left(), Direction::West);
    assert_eq!(Direction::North.turn_right(), Direction::East);
    assert_eq!(Direction::West.turn_right(), Direction::North);
    assert_eq!(Direction::South.turn_left(), Direction::East);
    // four rights make a full circle
    let d = Direction::East
        .turn_right()
        .turn_right()
        .turn_right()
        .turn_right();
    assert_eq!(d, Direction::East);
}

#[test]
fn step3_parse_simple_words() {
    assert_eq!(parse("left"), Some(Command::Left));
    assert_eq!(parse("RIGHT"), Some(Command::Right));
    assert_eq!(parse("  charge \n"), Some(Command::Charge));
    assert_eq!(parse("report"), Some(Command::Report));
}

#[test]
fn step3_parse_with_arguments() {
    assert_eq!(parse("forward 3"), Some(Command::Forward(3)));
    assert_eq!(parse("Forward 12"), Some(Command::Forward(12)));
    assert_eq!(
        parse("say Hello There"),
        Some(Command::Say("Hello There".to_string()))
    );
}

#[test]
fn step3_parse_nonsense_is_none() {
    assert_eq!(parse("dance"), None);
    assert_eq!(parse("forward"), None);
    assert_eq!(parse("forward three"), None);
    assert_eq!(parse("forward -2"), None);
    assert_eq!(parse("say"), None);
    assert_eq!(parse(""), None);
}

#[test]
fn step4_apply_moves_and_turns() {
    let mut r = Robot::new();
    r.apply(Command::Forward(3));
    r.apply(Command::Right);
    r.apply(Command::Forward(2));
    assert_eq!((r.x, r.y, r.facing, r.battery), (2, 3, Direction::East, 95));
}

#[test]
fn step4_apply_battery_rules() {
    let mut r = Robot::new();
    let msg = r.apply(Command::Forward(101));
    assert!(msg.contains("Not enough battery"), "got: {msg}");
    assert_eq!((r.x, r.y, r.battery), (0, 0, 100));
    r.apply(Command::Forward(60));
    r.apply(Command::Charge);
    assert_eq!(r.battery, 100);
}

#[test]
fn step4_apply_say() {
    let mut r = Robot::new();
    assert_eq!(r.apply(Command::Say("beep".into())), "Robbie says: beep");
}

#[test]
fn step5_status() {
    let mut r = Robot::new();
    r.apply(Command::Left);
    r.apply(Command::Forward(4));
    assert_eq!(r.status(), "(-4, 0) facing West, battery 96%");
    assert_eq!(r.apply(Command::Report), r.status());
}
