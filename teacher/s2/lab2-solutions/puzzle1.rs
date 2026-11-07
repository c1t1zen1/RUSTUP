// Puzzle 1 solution: borrow instead of move.
fn main() {
    let name = String::from("Robbie");
    let greeting = &name;
    println!("Greeting: hello, {greeting}!");
    println!("Name tag says: {name}");
}
