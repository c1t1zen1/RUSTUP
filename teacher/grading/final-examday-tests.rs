//! Exam-day tests for the final project: the charger tile and the drop
//! command. Kept secret until exam day, then copied into tests/ by the
//! grader alongside the hidden tests.
use robbie_mc::*;

fn one_bot(map: &str, start: (usize, usize), facing: &str, battery: u32, script: &str) -> Report {
    let m = Mission {
        map: map.to_string(),
        bots: vec![BotSpec {
            name: "Robbie".to_string(),
            start,
            facing: facing.to_string(),
            battery,
            script: script.to_string(),
        }],
    };
    run_mission(&m).unwrap()
}

#[test]
fn examday_charger_is_a_tile() {
    assert_eq!(Tile::from_char('C'), Some(Tile::Charger));
    assert_eq!(Tile::Charger.to_char(), 'C');
    let w = World::parse("#C.\n..P").unwrap();
    assert_eq!(w.tile(1, 0), Some(Tile::Charger));
    assert_eq!(w.render(), "#C.\n..P");
    assert_eq!(w.parts_left(), 1);
}

#[test]
fn examday_driving_onto_a_charger_refills() {
    let r = one_bot("...C...", (0, 0), "E", 5, "forward 3");
    assert_eq!((r.bots[0].end, r.bots[0].battery), ((3, 0), 5));
    let r = one_bot("...C....", (0, 0), "E", 5, "forward 7");
    assert_eq!(
        (r.bots[0].end, r.bots[0].battery, r.bots[0].flat),
        ((7, 0), 1, false)
    );
}

#[test]
fn examday_charger_lets_a_bot_go_further() {
    let without = one_bot("........", (0, 0), "E", 4, "forward 7");
    assert_eq!((without.bots[0].end, without.bots[0].flat), ((4, 0), true));
    let with = one_bot("...C....", (0, 0), "E", 4, "forward 7");
    assert_eq!(
        (with.bots[0].end, with.bots[0].battery, with.bots[0].flat),
        ((7, 0), 0, false)
    );
}

#[test]
fn examday_starting_on_or_turning_on_a_charger_does_not_charge() {
    let r = one_bot(
        "C...",
        (0, 0),
        "E",
        4,
        "forward 2\nleft\nleft\nforward 2\nright\nright\nwait",
    );
    // Driving back onto the charger refills it. The turns and the wait don't.
    assert_eq!((r.bots[0].end, r.bots[0].battery), ((0, 0), 4));
    let r = one_bot(
        "C...",
        (0, 0),
        "E",
        4,
        "forward 2\nleft\nleft\nforward 1\nwait",
    );
    assert_eq!((r.bots[0].end, r.bots[0].battery), ((1, 0), 1));
    let r = one_bot("C..", (0, 0), "E", 3, "wait\nleft");
    assert_eq!(r.bots[0].battery, 3);
}

#[test]
fn examday_bumping_into_a_charger_is_not_possible() {
    let r = one_bot("#C#", (1, 0), "E", 3, "forward");
    assert_eq!(
        (r.bots[0].end, r.bots[0].bumps, r.bots[0].battery),
        ((1, 0), 1, 2)
    );
}

#[test]
fn examday_drop_reads_and_puts_a_part_down() {
    assert_eq!(
        parse_script("DROP\n drop ").unwrap(),
        vec![Command::Drop, Command::Drop]
    );
    assert!(parse_script("drop 2").is_err());
    let r = one_bot("P...", (0, 0), "E", 9, "pick\nforward 2\ndrop");
    assert_eq!(
        (r.bots[0].parts, r.parts_collected, r.parts_left),
        (0, 0, 1)
    );
    assert_eq!(r.map, "..P.");
}

#[test]
fn examday_drop_needs_a_part_and_plain_floor() {
    let r = one_bot("..", (0, 0), "E", 9, "drop");
    assert_eq!((r.parts_left, r.map.as_str()), (0, ".."));
    let r = one_bot("PPC", (0, 0), "E", 9, "pick\nforward\ndrop\nforward\ndrop");
    assert_eq!((r.bots[0].parts, r.map.as_str()), (1, ".PC"));
}

#[test]
fn examday_flat_bot_cannot_drop() {
    let r = one_bot("P.", (0, 0), "E", 0, "pick\nforward\ndrop");
    assert!(r.bots[0].flat);
    assert_eq!((r.bots[0].parts, r.map.as_str()), (1, ".."));
}
