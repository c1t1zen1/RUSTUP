// Puzzle 7 (boss level): The First Word
// first_word() should return a slice of the input: the text up to the first space.
// Run: cargo run --bin puzzle7
// Goal: make first_word work on both a String and a string literal, with no new allocations.

fn first_word(text: &String) -> String {
    let mut out = String::new();
    for c in text.chars() {
        if c == ' ' {
            break;
        }
        out.push(c);
    }
    out
}

fn main() {
    let owned = String::from("rotate left 90");
    println!("{}", first_word(&owned));
    println!("{}", first_word("wave hello"));
}
