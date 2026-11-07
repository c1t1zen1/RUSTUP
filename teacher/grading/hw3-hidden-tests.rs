//! Homework 3 hidden tests. Copy into the student's tests/ folder before grading.
use command_language::parse::{Command, ParseError, parse_line};
use command_language::robot::{Direction, Robot, RobotError};
use command_language::{ScriptError, run_script};

#[test]
fn hidden_keywords_ignore_case_and_spaces() {
    assert_eq!(parse_line("  FORWARD   7  "), Ok(Command::Forward(7)));
    assert_eq!(parse_line("Report"), Ok(Command::Report));
    assert_eq!(
        parse_line("SAY Mixed Case Stays"),
        Ok(Command::Say("Mixed Case Stays".into()))
    );
}

#[test]
fn hidden_bad_arguments() {
    assert_eq!(
        parse_line("back -1"),
        Err(ParseError::BadNumber("-1".into()))
    );
    assert_eq!(
        parse_line("forward 2.5"),
        Err(ParseError::BadNumber("2.5".into()))
    );
    assert_eq!(parse_line("say"), Err(ParseError::MissingArgument("text")));
    assert_eq!(
        parse_line("say    "),
        Err(ParseError::MissingArgument("text"))
    );
    assert_eq!(
        parse_line("back"),
        Err(ParseError::MissingArgument("distance"))
    );
}

#[test]
fn hidden_zero_distance_is_fine() {
    let mut r = Robot::new();
    assert_eq!(r.apply(&Command::Forward(0)), Ok(None));
    assert_eq!((r.x, r.y, r.battery), (0, 0, 100));
}

#[test]
fn hidden_edge_of_grid() {
    let mut r = Robot::new();
    r.apply(&Command::Forward(10)).unwrap();
    assert_eq!(r.y, 10);
    assert_eq!(
        r.apply(&Command::Forward(1)),
        Err(RobotError::OutOfBounds { x: 0, y: 11 })
    );
    r.apply(&Command::Left).unwrap();
    assert_eq!(
        r.apply(&Command::Back(11)),
        Err(RobotError::OutOfBounds { x: 11, y: 10 })
    );
    assert_eq!(
        (r.x, r.y, r.facing, r.battery),
        (0, 10, Direction::West, 90)
    );
}

#[test]
fn hidden_battery_exactly_enough_then_not() {
    let mut r = Robot::new();
    for _ in 0..5 {
        r.apply(&Command::Forward(10)).unwrap();
        r.apply(&Command::Back(10)).unwrap();
    }
    assert_eq!((r.y, r.battery), (0, 0));
    assert_eq!(
        r.apply(&Command::Forward(1)),
        Err(RobotError::LowBattery { needed: 1, left: 0 })
    );
    r.apply(&Command::Charge).unwrap();
    assert_eq!(r.battery, 100);
}

#[test]
fn hidden_battery_checked_before_bounds() {
    let mut r = Robot::new();
    r.apply(&Command::Forward(5)).unwrap();
    r.apply(&Command::Back(5)).unwrap();
    // 90 left, asking for 95: battery wins even though it would also leave the grid
    assert_eq!(
        r.apply(&Command::Forward(95)),
        Err(RobotError::LowBattery {
            needed: 95,
            left: 90
        })
    );
}

#[test]
fn hidden_comments_blank_lines_and_line_numbers() {
    let script = "# warm up\n\n   # indented comment\nforward 1\n\nforward 50\nforward 50\n";
    let err = run_script(script).unwrap_err();
    assert_eq!(
        err,
        ScriptError::Robot {
            line: 6,
            error: RobotError::OutOfBounds { x: 0, y: 51 }
        }
    );
    assert_eq!(err.to_string(), "line 6: would leave the grid at (0, 51)");
}

#[test]
fn hidden_empty_and_comment_only_scripts() {
    let (r, said) = run_script("").unwrap();
    assert_eq!((r, said.len()), (Robot::new(), 0));
    let (_, said) = run_script("# nothing to see\n\n").unwrap();
    assert!(said.is_empty());
}

#[test]
fn hidden_first_error_wins() {
    let err = run_script("forward 2\nforward two\njump\n").unwrap_err();
    assert_eq!(err.to_string(), "line 2: 'two' is not a whole number");
}
