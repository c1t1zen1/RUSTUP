//! Your own tests. Write at least three more, each checking a rule from a doc
//! comment that the visible tests don't. They count toward your grade.
use swarm_sim::config::{Config, SimError};

#[test]
fn example_lowercase_moves_are_bad() {
    let text =
        r#"{ "floor": [[1]], "bots": [{ "name": "Robbie", "start": [0, 0], "route": "n" }] }"#;
    assert_eq!(
        Config::from_json(text),
        Err(SimError::BadMove {
            bot: "Robbie".to_string(),
            step: 'n'
        })
    );
}
