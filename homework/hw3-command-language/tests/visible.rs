//! Homework 3 visible tests. The grader adds more hidden ones, so test the edges yourself too.
use command_language::parse::{Command, ParseError, parse_line};
use command_language::robot::{Direction, Robot, RobotError};
use command_language::{ScriptError, run_script};

#[test]
fn parses_every_command() {
    assert_eq!(parse_line("forward 3"), Ok(Command::Forward(3)));
    assert_eq!(parse_line("back 1"), Ok(Command::Back(1)));
    assert_eq!(parse_line("left"), Ok(Command::Left));
    assert_eq!(parse_line("right"), Ok(Command::Right));
    assert_eq!(parse_line("say hi"), Ok(Command::Say("hi".into())));
    assert_eq!(parse_line("charge"), Ok(Command::Charge));
    assert_eq!(parse_line("report"), Ok(Command::Report));
}

#[test]
fn parse_errors_say_what_went_wrong() {
    assert_eq!(parse_line("   "), Err(ParseError::Empty));
    assert_eq!(
        parse_line("dance"),
        Err(ParseError::UnknownCommand("dance".into()))
    );
    assert_eq!(
        parse_line("forward"),
        Err(ParseError::MissingArgument("distance"))
    );
    assert_eq!(
        parse_line("forward lots"),
        Err(ParseError::BadNumber("lots".into()))
    );
}

#[test]
fn parse_error_messages() {
    assert_eq!(ParseError::Empty.to_string(), "empty line");
    assert_eq!(
        ParseError::UnknownCommand("dance".into()).to_string(),
        "unknown command 'dance'"
    );
    assert_eq!(
        ParseError::MissingArgument("text").to_string(),
        "missing text"
    );
    assert_eq!(
        ParseError::BadNumber("x".into()).to_string(),
        "'x' is not a whole number"
    );
}

#[test]
fn robot_moves_and_turns() {
    let mut r = Robot::new();
    r.apply(&Command::Forward(3)).unwrap();
    r.apply(&Command::Right).unwrap();
    r.apply(&Command::Back(2)).unwrap();
    assert_eq!(
        (r.x, r.y, r.facing, r.battery),
        (-2, 3, Direction::East, 95)
    );
}

#[test]
fn robot_refuses_and_stays_put() {
    let mut r = Robot::new();
    assert_eq!(
        r.apply(&Command::Forward(11)),
        Err(RobotError::OutOfBounds { x: 0, y: 11 })
    );
    assert_eq!((r.x, r.y, r.battery), (0, 0, 100));
    assert_eq!(
        RobotError::LowBattery { needed: 5, left: 2 }.to_string(),
        "needs 5% battery, has 2%"
    );
}

#[test]
fn say_and_report_produce_output() {
    let mut r = Robot::new();
    assert_eq!(
        r.apply(&Command::Say("beep".into())),
        Ok(Some("Robbie says: beep".into()))
    );
    assert_eq!(
        r.apply(&Command::Report),
        Ok(Some("(0, 0) facing North, battery 100%".into()))
    );
    assert_eq!(r.apply(&Command::Left), Ok(None));
}

#[test]
fn runs_a_script() {
    let (r, said) = run_script("say hello\nforward 2\nleft\nforward 1\nreport\n").unwrap();
    assert_eq!((r.x, r.y), (-1, 2));
    assert_eq!(
        said,
        vec!["Robbie says: hello", "(-1, 2) facing West, battery 97%"]
    );
}

#[test]
fn script_errors_name_the_line() {
    let err = run_script("forward 1\n\njump\n").unwrap_err();
    assert_eq!(
        err,
        ScriptError::Parse {
            line: 3,
            error: ParseError::UnknownCommand("jump".into())
        }
    );
    assert_eq!(err.to_string(), "line 3: unknown command 'jump'");
}
