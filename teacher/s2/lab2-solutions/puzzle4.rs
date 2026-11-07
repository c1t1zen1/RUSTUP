// Puzzle 4 solution: finish using the shared borrow before the vector changes.
fn main() {
    let mut parts = vec![String::from("head"), String::from("arm")];
    let first = &parts[0];
    println!("first part: {first}");
    parts.push(String::from("tread"));
    println!("all parts: {parts:?}");
}
