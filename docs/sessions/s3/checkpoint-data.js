// Checkpoint 1: review of Sessions 1 to 3. type: "mc" (pick one) or "text" (typed answer, any of `answers`, case and spaces ignored).
window.QUIZ = {
 "id": "s3-cp1",
 "title": "Checkpoint 1: Sessions 1 to 3",
 "pass": 70,
 "questions": [
  {
   "type": "mc",
   "prompt": "Which command starts a new Rust project with a Cargo.toml and src/main.rs?",
   "options": [
    "cargo new robbie",
    "rustc new robbie",
    "rustup new robbie",
    "cargo build robbie"
   ],
   "answer": 0,
   "why": "cargo new makes the folder, the manifest, a hello-world main.rs and a git repo. (Session 1)"
  },
  {
   "type": "mc",
   "prompt": "Will this compile?",
   "options": [
    "Yes",
    "No: x isn't mut"
   ],
   "answer": 1,
   "why": "Variables are immutable by default. let mut x = 5; would fix it. (Session 1)",
   "code": "fn main() {\n    let x = 5;\n    x = 6;\n    println!(\"{x}\");\n}"
  },
  {
   "type": "text",
   "prompt": "What is <code>y</code>?",
   "answers": [
    "6"
   ],
   "why": "A block is an expression: its last line, with no semicolon, is its value. (Session 1)",
   "code": "let y = {\n    let a = 3;\n    a * 2\n};"
  },
  {
   "type": "mc",
   "prompt": "Which loop goes over 1 to 10, including 10?",
   "options": [
    "for n in 1..10",
    "for n in 1..=10",
    "loop n in 1 to 10",
    "while n in 1..=10"
   ],
   "answer": 1,
   "why": "1..10 stops before 10. 1..=10 includes it. (Session 1)"
  },
  {
   "type": "mc",
   "prompt": "What's going on here?",
   "options": [
    "An error: you can't change a variable's type",
    "Fine: the second let is a new variable that shadows the first",
    "It needs let mut",
    "s becomes an Option"
   ],
   "answer": 1,
   "why": "Shadowing makes a brand new variable with the same name, so its type can differ. (Session 1)",
   "code": "let s = \"42\";\nlet s: u32 = s.parse().unwrap();"
  },
  {
   "type": "mc",
   "prompt": "What happens here?",
   "options": [
    "Prints wrench",
    "Error E0382: a was moved into b",
    "Prints an empty line",
    "b is a copy, so both work"
   ],
   "answer": 1,
   "why": "A String owns heap memory, so let b = a moves it. Using a afterwards is E0382. (Session 2)",
   "code": "let a = String::from(\"wrench\");\nlet b = a;\nprintln!(\"{a} {b}\");"
  },
  {
   "type": "mc",
   "prompt": "Which of these is Copy?",
   "options": [
    "String",
    "Vec<i32>",
    "(i32, bool)",
    "Box<i32>"
   ],
   "answer": 2,
   "why": "A tuple of Copy types is Copy. The others own heap memory, so they move. (Session 2)"
  },
  {
   "type": "mc",
   "prompt": "The borrowing rule is...",
   "options": [
    "any number of & or exactly one &mut, never both",
    "one & and one &mut at a time",
    "any number of &mut",
    "only one & at a time"
   ],
   "answer": 0,
   "why": "Many readers or one writer. That one rule rules out data races. (Session 2)"
  },
  {
   "type": "mc",
   "prompt": "The best parameter type for text a function only reads:",
   "options": [
    "String",
    "&String",
    "&str",
    "&mut String"
   ],
   "answer": 2,
   "why": "&amp;str accepts a &amp;String, a slice or a literal, and doesn't take ownership. (Session 2)"
  },
  {
   "type": "text",
   "prompt": "What does this evaluate to?",
   "answers": [
    "rob",
    "\"rob\""
   ],
   "why": "Slices take byte positions, start included and end excluded: 0, 1 and 2. (Session 2)",
   "code": "&\"robbie\"[0..3]"
  },
  {
   "type": "mc",
   "prompt": "A method that only reads its struct should take...",
   "options": [
    "self",
    "&self",
    "&mut self",
    "nothing"
   ],
   "answer": 1,
   "why": "&amp;self looks, &amp;mut self touches, self takes. (Session 3)"
  },
  {
   "type": "mc",
   "prompt": "A function can fail and the caller needs to know why. What should it return?",
   "options": [
    "Option<T>",
    "Result<T, E>",
    "bool",
    "()"
   ],
   "answer": 1,
   "why": "Result carries the reason in Err. Option only says \"nothing\". (Session 3)"
  },
  {
   "type": "mc",
   "prompt": "Will this compile?",
   "options": [
    "Yes",
    "No: 101 to 255 aren't covered"
   ],
   "answer": 1,
   "why": "A u8 goes up to 255, so the match isn't exhaustive (E0004). Add a _ arm. (Session 3)",
   "code": "fn label(battery: u8) -> &'static str {\n    match battery {\n        0..=19 => \"low\",\n        20..=100 => \"ok\",\n    }\n}"
  },
  {
   "type": "mc",
   "prompt": "What does <code>cargo clippy -- -D warnings</code> do?",
   "options": [
    "Formats your code",
    "Runs the linter and treats every warning as an error",
    "Runs the tests",
    "Publishes the crate"
   ],
   "answer": 1,
   "why": "Clippy finds code that works but could be clearer or faster. -D warnings makes the build fail until it's clean, which is how homework is graded. (Session 1)"
  },
  {
   "type": "mc",
   "prompt": "Inside <code>fn load(s: &amp;str) -&gt; Result&lt;u32, ParseIntError&gt;</code>, which line is right?",
   "options": [
    "let n: u32 = s.parse()?;",
    "let n: u32 = s.parse();",
    "let n: u32 = s.parse()!;",
    "let n: u32 = try s.parse();"
   ],
   "answer": 0,
   "why": "parse gives a Result; ? unwraps Ok or returns the Err from load. (Session 3)"
  }
 ]
};
