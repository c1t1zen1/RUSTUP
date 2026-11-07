// Puzzle 4: The Parts List
// Robbie looks at his first part, then bolts on a new one, then reads the first part again.
// Run: cargo run --bin puzzle4
// Goal: print the first part and then the full list. Reorder lines; don't add .clone().

fn main() {
    let mut parts = vec![String::from("head"), String::from("arm")];
    let first = &parts[0];
    parts.push(String::from("tread"));
    println!("first part: {first}");
    println!("all parts: {parts:?}");
}
