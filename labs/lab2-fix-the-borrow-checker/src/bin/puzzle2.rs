// Puzzle 2: The Shouting Function
// shout() only needs to LOOK at the text, but it takes the whole String away.
// Run: cargo run --bin puzzle2
// Goal: change shout's parameter type so both calls work. Don't touch main.

fn shout(text: String) -> usize {
    println!("{}!!!", text.to_uppercase());
    text.len()
}

fn main() {
    let line = String::from("battery low");
    let a = shout(line);
    let b = shout(line);
    println!("shouted {} characters", a + b);
}
