//! Hidden tests for the final project, Robbie Mission Control.
//! Copy into the student's tests/ folder before grading (the grader does it
//! for you with --hidden). Every rule here is written down in the project
//! README; these just check the corners.
use robbie_mc::*;

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

fn temp(name: &str, body: &str) -> String {
    let dir = std::env::temp_dir().join(format!("robbie-mc-hidden-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    std::fs::write(&path, body).unwrap();
    path.to_string_lossy().into_owned()
}

#[test]
fn hidden_map_skips_blank_lines_but_counts_them() {
    let w = World::parse("\n...   \n\n.P.\n\n").unwrap();
    assert_eq!((w.width(), w.height()), (3, 2));
    assert_eq!(w.tile(1, 1), Some(Tile::Part));
    assert_eq!(
        World::parse("\n...\n\n.Q.").unwrap_err(),
        McError::UnknownTile {
            line: 4,
            col: 2,
            ch: 'Q'
        }
    );
    assert_eq!(
        World::parse("...\n\n....").unwrap_err(),
        McError::RaggedMap { line: 3 }
    );
    assert_eq!(World::parse("\n  \n\n").unwrap_err(), McError::EmptyMap);
}

#[test]
fn hidden_map_renders_back() {
    let text = "#..#\n.PP.\n####";
    assert_eq!(World::parse(text).unwrap().render(), text);
}

#[test]
fn hidden_script_is_forgiving_about_spacing_and_case() {
    let cmds = parse_script("  FORWARD\t 2 \n\n   # indented note\nRiGhT\n").unwrap();
    assert_eq!(cmds, vec![Command::Forward(2), Command::Right]);
    assert_eq!(parse_script("").unwrap(), vec![]);
    assert_eq!(parse_script("# only a note").unwrap(), vec![]);
}

#[test]
fn hidden_script_errors_count_every_line() {
    for (src, line, text) in [
        ("left\n\n# note\nforward two", 4, "forward two"),
        ("forward 2 3", 1, "forward 2 3"),
        ("pick\n  spin  ", 2, "spin"),
        ("forward 4294967296", 1, "forward 4294967296"),
    ] {
        assert_eq!(
            parse_script(src).unwrap_err(),
            McError::BadCommand {
                line,
                text: text.to_string()
            },
            "script {src:?}"
        );
    }
    assert_eq!(
        parse_script("forward 4294967295").unwrap(),
        vec![Command::Forward(u32::MAX)]
    );
}

#[test]
fn hidden_bump_costs_battery_and_ends_the_forward() {
    let mut world = World::parse("...#....").unwrap();
    let mut bot = Bot::new("Robbie", 0, 0, Facing::East, 10);
    bot.run(&mut world, Command::Forward(6));
    assert_eq!((bot.x, bot.steps, bot.bumps, bot.battery), (2, 2, 1, 7));
}

#[test]
fn hidden_every_edge_is_a_bump() {
    let mut world = World::parse("..\n..").unwrap();
    for (facing, x, y) in [
        (Facing::North, 0, 0),
        (Facing::West, 0, 1),
        (Facing::South, 1, 1),
        (Facing::East, 1, 0),
    ] {
        let mut bot = Bot::new("Edge", x, y, facing, 3);
        bot.run(&mut world, Command::Forward(1));
        assert_eq!(
            (bot.x, bot.y, bot.bumps, bot.battery),
            (x, y, 1, 2),
            "{facing:?}"
        );
    }
}

#[test]
fn hidden_exact_battery_is_not_flat_until_the_next_move() {
    let mut world = World::parse("......").unwrap();
    let mut bot = Bot::new("Robbie", 0, 0, Facing::East, 3);
    bot.run(&mut world, Command::Forward(3));
    assert_eq!((bot.x, bot.battery, bot.flat), (3, 0, false));
    bot.run(&mut world, Command::Right);
    assert_eq!(bot.facing, Facing::South, "turning is free");
    bot.run(&mut world, Command::Forward(1));
    assert_eq!(
        (bot.y, bot.bumps, bot.flat),
        (0, 0, true),
        "flat, not a bump"
    );
}

#[test]
fn hidden_flat_bot_cannot_pick() {
    let mut world = World::parse(".P").unwrap();
    let mut bot = Bot::new("Robbie", 0, 0, Facing::East, 1);
    bot.run(&mut world, Command::Forward(1));
    bot.run(&mut world, Command::Forward(1));
    assert!(bot.flat);
    bot.run(&mut world, Command::Pick);
    assert_eq!((bot.parts, world.parts_left()), (0, 1));
}

#[test]
fn hidden_checks_run_in_the_documented_order() {
    // The map is checked before any bot.
    let m = mission("..\n.", &[("A", (9, 9), "?", 1, "jump")]);
    assert_eq!(run_mission(&m).unwrap_err(), McError::RaggedMap { line: 2 });
    // Bots in order: the second bot's duplicate name beats the third's bad start.
    let m = mission(
        "...",
        &[
            ("A", (0, 0), "E", 1, ""),
            ("A", (1, 0), "E", 1, ""),
            ("B", (7, 0), "E", 1, ""),
        ],
    );
    assert_eq!(
        run_mission(&m).unwrap_err(),
        McError::DuplicateBot("A".to_string())
    );
    // Within one bot: off the map, then the facing, then the script.
    let m = mission("...", &[("A", (3, 0), "up", 1, "jump")]);
    assert_eq!(
        run_mission(&m).unwrap_err(),
        McError::OffMap {
            bot: "A".to_string()
        }
    );
    let m = mission("...", &[("A", (0, 0), "up", 1, "jump")]);
    assert_eq!(
        run_mission(&m).unwrap_err(),
        McError::BadFacing {
            bot: "A".to_string(),
            facing: "up".to_string()
        }
    );
    // Names are case-sensitive.
    let m = mission(
        "...",
        &[
            ("Robbie", (0, 0), "E", 1, ""),
            ("robbie", (0, 0), "e", 1, ""),
        ],
    );
    assert!(run_mission(&m).is_ok());
}

#[test]
fn hidden_every_message() {
    let cases: Vec<(McError, &str)> = vec![
        (McError::EmptyMap, "the map is empty"),
        (
            McError::UnknownTile {
                line: 3,
                col: 7,
                ch: '~',
            },
            "map line 3, column 7: unknown tile '~'",
        ),
        (
            McError::RaggedMap { line: 2 },
            "map line 2 is a different length from the first row",
        ),
        (
            McError::BadCommand {
                line: 5,
                text: "hop".into(),
            },
            "script line 5: can't understand 'hop'",
        ),
        (
            McError::BotScript {
                bot: "Zippy".into(),
                line: 1,
                text: "hop".into(),
            },
            "Zippy's script line 1: can't understand 'hop'",
        ),
        (
            McError::DuplicateBot("Dusty".into()),
            "two bots are called Dusty",
        ),
        (
            McError::OffMap {
                bot: "Dusty".into(),
            },
            "Dusty starts off the map",
        ),
        (
            McError::InWall {
                bot: "Dusty".into(),
            },
            "Dusty starts inside a wall",
        ),
        (
            McError::BadFacing {
                bot: "Dusty".into(),
                facing: "Up".into(),
            },
            "Dusty has an unknown facing 'Up'",
        ),
        (McError::Json("eof".into()), "can't read mission: eof"),
    ];
    for (err, msg) in cases {
        assert_eq!(err.to_string(), msg);
    }
}

#[test]
fn hidden_json_problems_are_errors_not_panics() {
    assert!(matches!(
        Mission::from_json(r#"{"map": "..."}"#),
        Err(McError::Json(_))
    ));
    assert!(matches!(
        Mission::from_json(r#"{"map": 5, "bots": []}"#),
        Err(McError::Json(_))
    ));
    let m = Mission::from_json(r#"{"map": "..P\nP..", "bots": []}"#).unwrap();
    let r = run_mission(&m).unwrap();
    assert!(r.bots.is_empty());
    assert_eq!((r.parts_collected, r.parts_left), (0, 2));
}

#[test]
fn hidden_start_on_a_part() {
    let m = mission("P#P", &[("Robbie", (0, 0), "E", 5, "pick\nforward\npick")]);
    let r = run_mission(&m).unwrap();
    assert_eq!((r.bots[0].parts, r.bots[0].bumps, r.parts_left), (1, 1, 1));
    assert_eq!(r.map, ".#P");
}

#[test]
fn hidden_text_report_last_line_and_flat_marker() {
    let m = mission("P..", &[("Robbie", (0, 0), "E", 1, "pick\nforward 2")]);
    let text = run_mission(&m).unwrap().to_text();
    assert!(text.starts_with("Mission report"));
    assert!(text.contains("FLAT"));
    assert!(
        text.trim_end()
            .ends_with("Parts collected: 1. Parts left: 0.")
    );
}

#[test]
fn hidden_contested_parts_keep_their_totals() {
    let map = "P.P.P\n.....\nP.P.P";
    let m = mission(
        map,
        &[
            (
                "A",
                (0, 0),
                "E",
                30,
                "pick\nforward 2\npick\nforward 2\npick",
            ),
            (
                "B",
                (4, 0),
                "W",
                30,
                "pick\nforward 2\npick\nforward 2\npick",
            ),
            (
                "C",
                (0, 2),
                "E",
                30,
                "pick\nforward 2\npick\nforward 2\npick",
            ),
            (
                "D",
                (4, 2),
                "W",
                30,
                "pick\nforward 2\npick\nforward 2\npick",
            ),
        ],
    );
    let one = run_mission(&m).unwrap();
    for _ in 0..20 {
        let fleet = run_fleet(&m).unwrap();
        assert_eq!((fleet.parts_collected, fleet.parts_left), (6, 0));
        assert_eq!(fleet.map, one.map);
        for (a, b) in fleet.bots.iter().zip(&one.bots) {
            assert_eq!(
                (&a.name, a.end, a.steps, a.battery),
                (&b.name, b.end, b.steps, b.battery)
            );
        }
    }
}

#[test]
fn hidden_big_fleet_is_steady() {
    let map = ".........\n".repeat(8);
    let bots: Vec<(String, usize)> = (0..8).map(|i| (format!("Bot{i}"), i)).collect();
    let specs: Vec<Spec> = bots
        .iter()
        .map(|(n, i)| {
            (
                n.as_str(),
                (*i, *i),
                "E",
                40,
                "forward 3\nright\nforward 9\nleft\nleft\nforward 2",
            )
        })
        .collect();
    let m = mission(&map, &specs);
    let one = run_mission(&m).unwrap();
    for _ in 0..20 {
        assert_eq!(run_fleet(&m).unwrap(), one);
    }
}

#[test]
fn hidden_cli_runs_both_kinds_of_file() {
    let mission_file = temp(
        "m.json",
        r#"{"map": "..P", "bots": [{"name": "Robbie", "start": [0, 0], "facing": "E", "battery": 9, "script": "forward 2\npick"}]}"#,
    );
    let out = run_cli(&[mission_file.clone()]).unwrap();
    assert!(out.contains("Parts collected: 1. Parts left: 0."));
    let fleet = run_cli(&[mission_file.clone(), "--fleet".into(), "--json".into()]).unwrap();
    let v: serde_json::Value = serde_json::from_str(&fleet).unwrap();
    assert_eq!(v["bots"][0]["end"], serde_json::json!([2, 0]));

    let map = temp("map.txt", "###\n#.P\n###\n");
    let script = temp("go.txt", "forward\npick\n");
    let v: serde_json::Value =
        serde_json::from_str(&run_cli(&[map.clone(), script.clone(), "--json".into()]).unwrap())
            .unwrap();
    assert_eq!(v["bots"][0]["name"], "Robbie");
    assert_eq!(v["bots"][0]["end"], serde_json::json!([2, 1]));
    assert_eq!(v["bots"][0]["battery"], 49);
}

#[test]
fn hidden_cli_errors_are_values() {
    assert!(matches!(run_cli(&[]), Err(McError::Usage(_))));
    assert!(matches!(
        run_cli(&["a".into(), "b".into(), "c".into()]),
        Err(McError::Usage(_))
    ));
    assert!(matches!(
        run_cli(&["--loud".into()]),
        Err(McError::Usage(_))
    ));
    assert!(matches!(
        run_cli(&["/no/such/robbie.json".into()]),
        Err(McError::Io(_))
    ));
    let map = temp("bad-map.txt", "..\n.");
    let script = temp("ok.txt", "left");
    assert_eq!(
        run_cli(&[map, script]).unwrap_err(),
        McError::RaggedMap { line: 2 }
    );
    let map = temp("good-map.txt", "..");
    let script = temp("bad.txt", "left\nmoonwalk");
    assert_eq!(
        run_cli(&[map, script]).unwrap_err(),
        McError::BadCommand {
            line: 2,
            text: "moonwalk".into()
        }
    );
}
