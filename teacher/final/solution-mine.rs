//! A reference set of student tests: eleven rules worth pinning down.
use robbie_mc::*;

fn report(map: &str, battery: u32, script: &str) -> Report {
    let m = Mission {
        map: map.to_string(),
        bots: vec![BotSpec {
            name: "Robbie".to_string(),
            start: (0, 0),
            facing: "E".to_string(),
            battery,
            script: script.to_string(),
        }],
    };
    run_mission(&m).unwrap()
}

#[test]
fn a_bot_on_a_part_can_pick_it_without_moving() {
    let mut world = World::parse("P").unwrap();
    let mut robbie = Bot::new("Robbie", 0, 0, Facing::North, 0);
    robbie.run(&mut world, Command::Pick);
    assert_eq!(robbie.parts, 1);
}

#[test]
fn four_rights_make_a_circle() {
    let f = Facing::West;
    assert_eq!(f.right().right().right().right(), f);
    assert_eq!(f.left().right(), f);
}

#[test]
fn trailing_spaces_in_a_map_are_ignored() {
    assert_eq!(World::parse("..  \n..").unwrap().width(), 2);
}

#[test]
fn line_numbers_count_blank_lines() {
    assert_eq!(
        World::parse("\n\n.x").unwrap_err(),
        McError::UnknownTile {
            line: 3,
            col: 2,
            ch: 'x'
        }
    );
}

#[test]
fn forward_with_no_number_is_one() {
    assert_eq!(parse_script("forward").unwrap(), vec![Command::Forward(1)]);
}

#[test]
fn forward_zero_is_not_a_command() {
    assert_eq!(
        parse_script("forward 0").unwrap_err().to_string(),
        "script line 1: can't understand 'forward 0'"
    );
}

#[test]
fn a_bump_still_costs_battery() {
    let r = report(".#", 5, "forward 3");
    assert_eq!(
        (r.bots[0].battery, r.bots[0].bumps, r.bots[0].steps),
        (4, 1, 0)
    );
}

#[test]
fn running_out_exactly_is_not_flat() {
    let r = report("...", 2, "forward 2");
    assert!(!r.bots[0].flat);
    let r = report("...", 2, "forward 2\nleft\nforward");
    assert!(r.bots[0].flat);
}

#[test]
fn picking_twice_gets_one_part() {
    let r = report("P", 1, "pick\npick");
    assert_eq!(
        (r.parts_collected, r.parts_left, r.map.as_str()),
        (1, 0, ".")
    );
}

#[test]
fn a_mission_without_bots_still_reports() {
    let r = run_mission(&Mission {
        map: "PP".into(),
        bots: vec![],
    })
    .unwrap();
    assert_eq!(r.parts_left, 2);
}

#[test]
fn the_cli_wants_a_file() {
    assert!(matches!(run_cli(&[]), Err(McError::Usage(_))));
}
