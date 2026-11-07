// Puzzle 5 solution: one &mut at a time. Each borrow ends after its last use.
fn main() {
    let mut torque = 10;
    let mechanic_a = &mut torque;
    *mechanic_a += 1;
    let mechanic_b = &mut torque;
    *mechanic_b += 1;
    println!("torque: {torque}");
}
