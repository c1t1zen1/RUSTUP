//! Hidden tests for Homework 5: Swarm Sim.
//! Copy into the student's tests/ folder as tests/hidden.rs, then: cargo test --test hidden
use swarm_sim::config::{Config, SimError};
use swarm_sim::sim;
use swarm_sim::simulate_json;

fn one_bot(floor: &str, start: &str, route: &str) -> String {
    format!(
        r#"{{ "floor": {floor}, "bots": [{{ "name": "Robbie", "start": {start}, "route": "{route}" }}] }}"#
    )
}

#[test]
fn every_message_matches_the_doc_comment() {
    assert_eq!(SimError::EmptyFloor.to_string(), "the floor is empty");
    assert_eq!(
        SimError::RaggedFloor { row: 2 }.to_string(),
        "row 2 is a different length from row 0"
    );
    assert_eq!(
        SimError::DuplicateBot("Robbie".into()).to_string(),
        "two bots are called Robbie"
    );
    assert_eq!(
        SimError::Parse("eof".into()).to_string(),
        "can't read config: eof"
    );
}

#[test]
fn a_row_with_no_tiles_is_an_empty_floor() {
    let text = r#"{ "floor": [[]], "bots": [] }"#;
    assert_eq!(Config::from_json(text), Err(SimError::EmptyFloor));
}

#[test]
fn the_first_ragged_row_is_reported() {
    let text = r#"{ "floor": [[1, 1], [1, 1, 1], [1]], "bots": [] }"#;
    assert_eq!(
        Config::from_json(text),
        Err(SimError::RaggedFloor { row: 1 })
    );
}

#[test]
fn checks_run_in_the_documented_order() {
    // Ragged floor beats a bad bot.
    let text =
        r#"{ "floor": [[1, 1], [1]], "bots": [{ "name": "R", "start": [9, 9], "route": "Q" }] }"#;
    assert_eq!(
        Config::from_json(text),
        Err(SimError::RaggedFloor { row: 1 })
    );
    // For one bot: off the floor beats a bad move.
    let text = one_bot("[[1]]", "[0, 1]", "Q");
    assert_eq!(
        Config::from_json(&text),
        Err(SimError::OffFloor {
            bot: "Robbie".into()
        })
    );
    // The first bad bot wins, and a duplicate is the second one.
    let text = r#"{ "floor": [[1, 1]], "bots": [
        { "name": "Zippy", "start": [0, 0], "route": "" },
        { "name": "Zippy", "start": [5, 0], "route": "" },
        { "name": "Dusty", "start": [0, 0], "route": "!" }
    ] }"#;
    assert_eq!(
        Config::from_json(text),
        Err(SimError::DuplicateBot("Zippy".into()))
    );
}

#[test]
fn names_that_differ_in_case_are_different_bots() {
    let text = r#"{ "floor": [[1, 1]], "bots": [
        { "name": "zippy", "start": [0, 0], "route": "" },
        { "name": "Zippy", "start": [1, 0], "route": "" }
    ] }"#;
    assert!(Config::from_json(text).is_ok());
}

#[test]
fn a_tile_never_goes_below_zero() {
    let text = one_bot("[[1, 0]]", "[0, 0]", "EWEWEW");
    let report = sim::run(&Config::from_json(&text).unwrap());
    assert_eq!(report.floor, vec![vec![0, 0]]);
    assert_eq!((report.cleaned, report.dirt_left), (1, 0));
}

#[test]
fn shared_tiles_give_the_same_totals_every_run() {
    // Eight bots crowd a 3x3 floor. Who cleans what changes between runs; the totals don't.
    let routes = ["NESW", "EESS", "SSNN", "WWEE", "NNSS", "ESWN", "SEN", "WNE"];
    let bots: Vec<String> = routes
        .iter()
        .enumerate()
        .map(|(i, r)| format!(r#"{{ "name": "bot{i}", "start": [1, 1], "route": "{r}" }}"#))
        .collect();
    let text = format!(
        r#"{{ "floor": [[2, 2, 2], [2, 9, 2], [2, 2, 2]], "bots": [{}] }}"#,
        bots.join(",")
    );
    let config = Config::from_json(&text).unwrap();
    let first = sim::run(&config);
    assert_eq!(first.cleaned + first.dirt_left, 25);
    for _ in 0..20 {
        assert_eq!(sim::run(&config), first);
    }
}

#[test]
fn no_bots_cleans_nothing() {
    let report = sim::run(&Config::from_json(r#"{ "floor": [[4, 5]], "bots": [] }"#).unwrap());
    assert!(report.bots.is_empty());
    assert_eq!((report.cleaned, report.dirt_left), (0, 9));
}

#[test]
fn bad_config_never_reaches_the_sim() {
    let err = simulate_json(&one_bot("[[1]]", "[0, 0]", "NE S")).unwrap_err();
    assert_eq!(
        err,
        SimError::BadMove {
            bot: "Robbie".into(),
            step: ' '
        }
    );
    assert!(matches!(simulate_json("[]"), Err(SimError::Parse(_))));
}
