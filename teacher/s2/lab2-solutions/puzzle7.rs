// Puzzle 7 solution: take &str and return a slice of it. No new String is allocated.
fn first_word(text: &str) -> &str {
    for (i, b) in text.bytes().enumerate() {
        if b == b' ' {
            return &text[..i];
        }
    }
    text
}

fn main() {
    let owned = String::from("rotate left 90");
    println!("{}", first_word(&owned));
    println!("{}", first_word("wave hello"));
}
