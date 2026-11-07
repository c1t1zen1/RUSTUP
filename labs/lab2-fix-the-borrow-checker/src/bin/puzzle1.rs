// Puzzle 1: The Name Tag
// Robbie lends his name tag to a greeting... and then wants to read it again.
// Run: cargo run --bin puzzle1
// Goal: make it compile and print both lines, WITHOUT using .clone().

fn main() {
    let name = String::from("Robbie");
    let greeting = name;
    println!("Greeting: hello, {greeting}!");
    println!("Name tag says: {name}");
}
