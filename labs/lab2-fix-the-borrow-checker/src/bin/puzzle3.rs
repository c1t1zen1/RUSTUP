// Puzzle 3: The Recharge Station
// recharge() is supposed to change the battery level in place.
// Run: cargo run --bin puzzle3
// Goal: the program should print 70. Fix the function AND the call.

fn recharge(level: &u32) {
    *level += 30;
}

fn main() {
    let battery = 40;
    recharge(&battery);
    println!("battery: {battery}");
}
