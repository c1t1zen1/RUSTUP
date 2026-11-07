// Quiz 1: Boot Sequence. type: "mc" (pick one) or "text" (typed answer, any of `answers`, case and spaces ignored).
window.QUIZ = {
  id: "s1", title: "Quiz 1: Boot Sequence", pass: 70,
  questions: [
    { type: "mc", prompt: "What does <code>cargo check</code> do?",
      options: ["Compiles and runs the program", "Type-checks the code without producing a program", "Formats your code", "Runs the tests"],
      answer: 1, why: "cargo check stops before generating a binary, which makes it the fastest way to find compile errors." },
    { type: "mc", prompt: "Will this compile?", code: "let speed = 3;\nspeed += 1;",
      options: ["Yes", "No"], answer: 1, why: "speed is immutable. It needs let mut speed = 3;" },
    { type: "mc", prompt: "What does this print?", code: "let x = 5;\nlet x = x + 1;\n{\n    let x = x * 2;\n    println!(\"{x}\");\n}\nprintln!(\"{x}\");",
      options: ["12 then 12", "12 then 6", "6 then 6", "It doesn't compile"], answer: 1,
      why: "The inner let x shadows x only inside the braces. After the block ends, the outer x (6) is visible again." },
    { type: "mc", prompt: "With no other hints, what type does <code>let n = 42;</code> get?",
      options: ["i64", "u32", "i32", "usize"], answer: 2, why: "i32 is Rust's default integer type." },
    { type: "mc", prompt: "Which of these types can hold the value -5?",
      options: ["u8", "usize", "i16", "char"], answer: 2, why: "Only signed integer types (i8, i16, i32 ...) can go negative. i is for 'I can go negative'." },
    { type: "mc", prompt: "Will this compile?", code: "fn double(n: i32) -> i32 {\n    n * 2;\n}",
      options: ["Yes", "No"], answer: 1,
      why: "The semicolon mutes the value, so the body returns () instead of i32: 'mismatched types'. Remove the semicolon." },
    { type: "mc", prompt: "What is <code>250u8.saturating_add(10)</code>?",
      options: ["260", "4", "255", "It panics"], answer: 2, why: "saturating_ operations stop at the type's maximum, and u8 tops out at 255." },
    { type: "mc", prompt: "How many lines does this print?", code: "for i in 1..5 {\n    println!(\"{i}\");\n}",
      options: ["4", "5", "6", "It runs forever"], answer: 0, why: "1..5 stops before 5: it gives 1, 2, 3, 4. Use 1..=5 to include the 5." },
    { type: "text", prompt: "Which keyword lets a variable's value change?", answers: ["mut"],
      why: "let mut x = 0; makes x mutable. Let it be, unless you mut." },
    { type: "mc", prompt: "Will this compile?", code: "let label = if true { 1 } else { \"one\" };",
      options: ["Yes", "No"], answer: 1, why: "Both branches of an if expression must have the same type: here one is an integer and one is a &str." },
    { type: "text", prompt: "What value does <code>v</code> get?", code: "let v = loop {\n    break 2 * 21;\n};", answers: ["42"],
      why: "A loop hands back the value given to break: 2 * 21 = 42." },
    { type: "mc", prompt: "Which command fails if your code isn't formatted the standard way, without changing any files?",
      options: ["cargo fmt", "cargo fmt --check", "cargo clippy", "cargo build --release"], answer: 1,
      why: "cargo fmt rewrites files; cargo fmt --check only reports. The grader uses --check." }
  ]
};
