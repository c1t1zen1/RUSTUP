// Puzzle 2 solution: take &str, since shout only reads the text. &line coerces from &String to &str.
fn shout(text: &str) -> usize {
    println!("{}!!!", text.to_uppercase());
    text.len()
}

fn main() {
    let line = String::from("battery low");
    let a = shout(&line);
    let b = shout(&line);
    println!("shouted {} characters", a + b);
}
