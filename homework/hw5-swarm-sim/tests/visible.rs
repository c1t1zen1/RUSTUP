//! Visible tests for Homework 5. The hidden tests check the same rules harder.
use swarm_sim::config::{Config, SimError};
use swarm_sim::sim;
use swarm_sim::simulate_json;

const SMALL: &str = r#"{
  "floor": [[2, 0, 1], [0, 3, 0]],
  "bots": [{ "name": "Robbie", "start": [0, 0], "route": "ESE" }]
}"#;

#[test]
fn reads_a_good_config() {
    let config = Config::from_json(SMALL).unwrap();
    assert_eq!(config.floor, vec![vec![2, 0, 1], vec![0, 3, 0]]);
    assert_eq!(config.bots[0].name, "Robbie");
    assert_eq!(config.bots[0].start, (0, 0));
    assert_eq!((config.width(), config.height()), (3, 2));
}

#[test]
fn broken_json_is_a_parse_error() {
    let err = Config::from_json("{ \"floor\": [[1, 2]], ").unwrap_err();
    assert!(matches!(err, SimError::Parse(_)), "got {err:?}");
    assert!(err.to_string().starts_with("can't read config: "));
}

#[test]
fn floor_problems() {
    let empty = r#"{ "floor": [], "bots": [] }"#;
    assert_eq!(Config::from_json(empty), Err(SimError::EmptyFloor));
    let ragged = r#"{ "floor": [[1, 1], [1, 1], [1]], "bots": [] }"#;
    assert_eq!(
        Config::from_json(ragged),
        Err(SimError::RaggedFloor { row: 2 })
    );
}

#[test]
fn bot_problems() {
    let off =
        r#"{ "floor": [[1, 1]], "bots": [{ "name": "Zippy", "start": [2, 0], "route": "" }] }"#;
    let err = Config::from_json(off).unwrap_err();
    assert_eq!(err.to_string(), "Zippy starts off the floor");
    let bad =
        r#"{ "floor": [[1, 1]], "bots": [{ "name": "Dusty", "start": [0, 0], "route": "EX" }] }"#;
    let err = Config::from_json(bad).unwrap_err();
    assert_eq!(err.to_string(), "Dusty has a bad move: 'X'");
}

#[test]
fn one_bot_cleans_its_path() {
    let report = sim::run(&Config::from_json(SMALL).unwrap());
    // Start (0,0) cleans 1, E to (1,0) has none, S to (1,1) cleans 1, E to (2,1) has none.
    assert_eq!(report.floor, vec![vec![1, 0, 1], vec![0, 2, 0]]);
    assert_eq!((report.cleaned, report.dirt_left), (2, 4));
    assert_eq!(report.bots[0].end, (2, 1));
    assert_eq!((report.bots[0].moves, report.bots[0].bumps), (3, 0));
}

#[test]
fn edges_are_bumps() {
    let text = r#"{ "floor": [[1, 1]], "bots": [{ "name": "Robbie", "start": [0, 0], "route": "NWWES" }] }"#;
    let report = sim::run(&Config::from_json(text).unwrap());
    assert_eq!(report.bots[0].end, (1, 0));
    assert_eq!((report.bots[0].moves, report.bots[0].bumps), (1, 4));
}

#[test]
fn report_lists_bots_by_name() {
    let text = r#"{ "floor": [[0, 0, 0]], "bots": [
        { "name": "Zippy", "start": [0, 0], "route": "" },
        { "name": "Dusty", "start": [1, 0], "route": "" },
        { "name": "Robbie", "start": [2, 0], "route": "" }
    ] }"#;
    let report = sim::run(&Config::from_json(text).unwrap());
    let names: Vec<&str> = report.bots.iter().map(|b| b.name.as_str()).collect();
    assert_eq!(names, ["Dusty", "Robbie", "Zippy"]);
}

#[test]
fn json_in_json_out() {
    let out = simulate_json(SMALL).unwrap();
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["cleaned"], 2);
    assert_eq!(v["dirt_left"], 4);
    assert_eq!(v["bots"][0]["end"], serde_json::json!([2, 1]));
    assert_eq!(v["floor"][1][1], 2);
}
