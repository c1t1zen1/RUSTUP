//! Visible tests for the final project. The grader adds hidden tests of the
//! same kind, so read each rule in README.md, not just these examples.
use robbie_mc::*;

/// name, start, facing, battery, script
type Spec<'a> = (&'a str, (usize, usize), &'a str, u32, &'a str);

fn mission(map: &str, bots: &[Spec]) -> Mission {
    Mission {
        map: map.to_string(),
        bots: bots
            .iter()
            .map(|&(name, start, facing, battery, script)| BotSpec {
                name: name.to_string(),
                start,
                facing: facing.to_string(),
                battery,
                script: script.to_string(),
            })
            .collect(),
    }
}

#[test]
fn map_reads_size_and_tiles() {
    let w = World::parse("#####\n#.P.#\n#####\n").unwrap();
    assert_eq!((w.width(), w.height()), (5, 3));
    assert_eq!(w.tile(2, 1), Some(Tile::Part));
    assert_eq!(w.tile(0, 0), Some(Tile::Wall));
    assert_eq!(w.tile(5, 0), None);
    assert_eq!(w.parts_left(), 1);
}

#[test]
fn map_errors_name_the_line() {
    assert_eq!(World::parse(""), Err(McError::EmptyMap));
    assert_eq!(
        World::parse("...\n.X.").unwrap_err(),
        McError::UnknownTile {
            line: 2,
            col: 2,
            ch: 'X'
        }
    );
    assert_eq!(
        World::parse("...\n..").unwrap_err(),
        McError::RaggedMap { line: 2 }
    );
}

#[test]
fn script_reads_every_command() {
    let cmds = parse_script("forward\nforward 3\nLeft\n right \n\n# a note\npick\nwait").unwrap();
    assert_eq!(
        cmds,
        vec![
            Command::Forward(1),
            Command::Forward(3),
            Command::Left,
            Command::Right,
            Command::Pick,
            Command::Wait
        ]
    );
}

#[test]
fn script_errors_quote_the_line() {
    assert_eq!(
        parse_script("left\njump").unwrap_err(),
        McError::BadCommand {
            line: 2,
            text: "jump".to_string()
        }
    );
    assert!(parse_script("forward 0").is_err());
    assert!(parse_script("forward -2").is_err());
}

#[test]
fn turning_goes_round() {
    assert_eq!(Facing::North.right(), Facing::East);
    assert_eq!(Facing::North.left(), Facing::West);
    assert_eq!(Facing::South.left().left(), Facing::North);
    assert_eq!(Facing::parse("w"), Some(Facing::West));
    assert_eq!(Facing::parse("up"), None);
}

#[test]
fn walls_and_edges_are_bumps() {
    let mut world = World::parse("..#").unwrap();
    let mut bot = Bot::new("Robbie", 0, 0, Facing::East, 10);
    bot.run(&mut world, Command::Forward(5));
    assert_eq!((bot.x, bot.steps, bot.bumps, bot.battery), (1, 1, 1, 8));
    bot.run(&mut world, Command::Left);
    bot.run(&mut world, Command::Forward(1));
    assert_eq!((bot.x, bot.y, bot.bumps), (1, 0, 2));
}

#[test]
fn flat_battery_stops_the_bot() {
    let mut world = World::parse(".....").unwrap();
    let mut bot = Bot::new("Robbie", 0, 0, Facing::East, 2);
    bot.run(&mut world, Command::Forward(4));
    assert_eq!((bot.x, bot.battery, bot.flat), (2, 0, true));
    bot.run(&mut world, Command::Left);
    assert_eq!(bot.facing, Facing::East, "a flat bot ignores everything");
}

#[test]
fn picking_takes_the_part() {
    let mut world = World::parse(".P").unwrap();
    let mut bot = Bot::new("Robbie", 0, 0, Facing::East, 5);
    bot.run(&mut world, Command::Pick);
    assert_eq!(bot.parts, 0);
    bot.run(&mut world, Command::Forward(1));
    bot.run(&mut world, Command::Pick);
    bot.run(&mut world, Command::Pick);
    assert_eq!((bot.parts, world.parts_left()), (1, 0));
}

#[test]
fn mission_runs_bots_in_order() {
    let m = mission(
        "P...",
        &[
            ("Robbie", (0, 0), "E", 10, "pick\nforward 3"),
            ("Zippy", (0, 0), "E", 10, "pick"),
        ],
    );
    let r = run_mission(&m).unwrap();
    assert_eq!(r.bots[0].end, (3, 0));
    assert_eq!(r.bots[0].parts, 1);
    assert_eq!(r.bots[1].parts, 0, "Robbie got there first");
    assert_eq!((r.parts_collected, r.parts_left), (1, 0));
    assert_eq!(r.map, "....");
}

#[test]
fn mission_checks_the_bots() {
    let m = mission(
        "...",
        &[
            ("Robbie", (0, 0), "E", 5, ""),
            ("Robbie", (1, 0), "E", 5, ""),
        ],
    );
    assert_eq!(
        run_mission(&m).unwrap_err(),
        McError::DuplicateBot("Robbie".to_string())
    );
    let m = mission("..#", &[("Robbie", (2, 0), "E", 5, "")]);
    assert_eq!(
        run_mission(&m).unwrap_err().to_string(),
        "Robbie starts inside a wall"
    );
    let m = mission("...", &[("Robbie", (0, 0), "E", 5, "left\nhop")]);
    assert_eq!(
        run_mission(&m).unwrap_err().to_string(),
        "Robbie's script line 2: can't understand 'hop'"
    );
}

#[test]
fn json_mission_and_report() {
    let m = Mission::from_json(
        r#"{"map": "..P", "bots": [{"name": "Robbie", "start": [0, 0], "facing": "E", "battery": 9, "script": "forward 2\npick"}]}"#,
    )
    .unwrap();
    let r = run_mission(&m).unwrap();
    let json: serde_json::Value = serde_json::from_str(&r.to_json()).unwrap();
    assert_eq!(json["bots"][0]["end"], serde_json::json!([2, 0]));
    assert_eq!(json["parts_collected"], 1);
    assert!(matches!(Mission::from_json("{"), Err(McError::Json(_))));
}

#[test]
fn fleet_matches_one_at_a_time() {
    let m = mission(
        "P....\n.....\n....P",
        &[
            (
                "Robbie",
                (0, 0),
                "E",
                20,
                "pick\nforward 4\nright\nforward 2\npick",
            ),
            (
                "Zippy",
                (4, 0),
                "S",
                20,
                "forward 2\nright\nforward 4\nright\nforward 2",
            ),
        ],
    );
    let one = run_mission(&m).unwrap();
    for _ in 0..10 {
        assert_eq!(run_fleet(&m).unwrap(), one);
    }
}
