//! Your own tests. The final project needs at least 10 passing tests here.
//! Test the rules you think are easiest to get wrong. One example to start:
use robbie_mc::*;

#[test]
fn a_bot_on_a_part_can_pick_it_without_moving() {
    let mut world = World::parse("P").unwrap();
    let mut robbie = Bot::new("Robbie", 0, 0, Facing::North, 0);
    robbie.run(&mut world, Command::Pick);
    assert_eq!(robbie.parts, 1);
}
