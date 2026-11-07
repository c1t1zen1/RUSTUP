// Puzzle 3 solution: the variable must be mut, and the borrow must be &mut.
fn recharge(level: &mut u32) {
    *level += 30;
}

fn main() {
    let mut battery = 40;
    recharge(&mut battery);
    println!("battery: {battery}");
}
