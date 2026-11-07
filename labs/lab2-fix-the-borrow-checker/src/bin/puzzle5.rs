// Puzzle 5: Two Wrenches, One Bolt
// Two mechanics grab the same bolt at the same time.
// Run: cargo run --bin puzzle5
// Goal: print "torque: 12". Keep both mechanics, just don't let them overlap.

fn main() {
    let mut torque = 10;
    let mechanic_a = &mut torque;
    let mechanic_b = &mut torque;
    *mechanic_a += 1;
    *mechanic_b += 1;
    println!("torque: {torque}");
}
