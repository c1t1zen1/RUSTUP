// Quiz 6: Ship It. type: "mc" (pick one) or "text" (typed answer, any of `answers`, case and spaces ignored).
window.QUIZ = {
 "id": "s6",
 "title": "Quiz 6: Ship It",
 "pass": 70,
 "questions": [
  {
   "type": "mc",
   "prompt": "What happens when you build this?",
   "options": [
    "Prints squeak three times",
    "Prints squeak once and ignores the 3",
    "Compile error: no rule of squeak! takes an argument",
    "It panics at runtime"
   ],
   "answer": 2,
   "why": "A macro matches what you pass against each arm. The only arm is (), so squeak!(3) has nothing to match and the build stops.",
   "code": "macro_rules! squeak {\n    () => {\n        println!(\"squeak\")\n    };\n}\n\nfn main() {\n    squeak!(3);\n}"
  },
  {
   "type": "text",
   "prompt": "What does this print?",
   "answers": [
    "3"
   ],
   "why": "Each call peels one item off and adds 1: 1 + 1 + 1 + count!() = 1 + 1 + 1 + 0.",
   "code": "macro_rules! count {\n    () => { 0 };\n    ($x:expr $(, $rest:expr)*) => {\n        1 + count!($($rest),*)\n    };\n}\n\nfn main() {\n    println!(\"{}\", count!(\"bolt\", \"spring\", \"elbow\"));\n}"
  },
  {
   "type": "mc",
   "prompt": "Which line needs an <code>unsafe</code> block?",
   "options": [
    "let p = &battery as *const i32;",
    "println!(\"{p:p}\"); (printing the address)",
    "println!(\"{}\", *p); (reading through the pointer)",
    "None of them"
   ],
   "answer": 2,
   "why": "Making a raw pointer and printing it are safe. Reading through it is one of unsafe's five powers, so it's E0133 without unsafe.",
   "code": "fn main() {\n    let battery = 87;\n    let p = &battery as *const i32;\n    println!(\"{p:p}\");\n    println!(\"{}\", *p);\n}"
  },
  {
   "type": "mc",
   "prompt": "What does an <code>unsafe</code> block switch off?",
   "options": [
    "The borrow checker",
    "All type checking inside it",
    "Nothing. It unlocks five extra powers, and you promise you used them correctly",
    "Bounds checks on slices"
   ],
   "answer": 2,
   "why": "Every normal check still runs inside unsafe. It only allows things like raw pointer reads and unsafe calls, and the SAFETY comment says why they're fine."
  },
  {
   "type": "mc",
   "prompt": "The two halves don't overlap, so why does this fail with E0499?",
   "options": [
    "Arrays can't be sliced twice",
    "Rust only sees two &mut borrows of joints at once, not which parts they cover",
    "left must be declared mut",
    "joints needs to be a Vec"
   ],
   "answer": 1,
   "why": "The borrow checker reasons about joints as a whole. split_at_mut(2) gives both halves safely, because std did the unsafe part once and checked it.",
   "code": "fn main() {\n    let mut joints = [5, 9, 2, 7];\n    let left = &mut joints[..2];\n    let right = &mut joints[2..];\n    left[0] = 0;\n    right[0] = 0;\n}"
  },
  {
   "type": "mc",
   "prompt": "In Rust 2024, this fails with E0133. What fixes it?",
   "options": [
    "Add #[no_mangle]",
    "Write unsafe { abs(-12) }, or declare it as safe fn abs inside the block",
    "Change \"C\" to \"Rust\"",
    "Use i64 instead of i32"
   ],
   "answer": 1,
   "why": "Rust can't check C code, so calling a foreign function is unsafe unless you mark it safe fn and take on that promise.",
   "code": "unsafe extern \"C\" {\n    fn abs(x: i32) -> i32;\n}\n\nfn main() {\n    println!(\"{}\", abs(-12));\n}"
  },
  {
   "type": "mc",
   "prompt": "Why can't you hand a <code>&amp;str</code> straight to C's <code>strlen</code>?",
   "options": [
    "C can't read UTF-8",
    "A &str has no 0 byte at the end, so C wouldn't know where it stops",
    "&str is too long for C",
    "strlen only takes numbers"
   ],
   "answer": 1,
   "why": "C strings end at a 0 byte. CString (owned) and &amp;CStr (borrowed) guarantee it, and c\"robbie\" makes a &amp;CStr literal."
  },
  {
   "type": "text",
   "prompt": "What does this print?",
   "answers": [
    "None 255"
   ],
   "why": "checked_add says None when 250 + 10 won't fit in a u8. saturating_add stops at the top, 255.",
   "code": "fn main() {\n    let battery: u8 = 250;\n    println!(\"{:?} {}\", battery.checked_add(10), battery.saturating_add(10));\n}"
  },
  {
   "type": "mc",
   "prompt": "Why wrap the input in <code>std::hint::black_box</code> when you time code?",
   "options": [
    "It makes the code faster",
    "So the optimiser can't work out the answer ahead of time and skip the work",
    "It turns on --release",
    "It hides the timer from other threads"
   ],
   "answer": 1,
   "why": "In a release build the compiler may compute a constant answer and delete the loop. black_box hides the value so the work really runs. Time in --release too."
  },
  {
   "type": "mc",
   "prompt": "Which command adds the target that builds Rust for the browser?",
   "options": [
    "rustup target add wasm32-unknown-unknown",
    "cargo install webassembly",
    "rustc --web",
    "cargo build --browser"
   ],
   "answer": 0,
   "why": "wasm32-unknown-unknown is the browser target. wasm-bindgen (or wasm-pack) then makes the JavaScript glue."
  },
  {
   "type": "mc",
   "prompt": "<code>cargo publish --dry-run</code> stops before uploading. What is crates.io most likely missing?",
   "options": [
    "A main.rs",
    "description and license in Cargo.toml",
    "A tests folder",
    "A .gitignore"
   ],
   "answer": 1,
   "why": "crates.io needs description and license (or license-file). repository, keywords and categories help people find it. The Lab 6 step 5 test checks them."
  },
  {
   "type": "mc",
   "prompt": "A reviewer reads your library's mission parser. Which one do they flag?",
   "options": [
    "It returns Result<Mission, McError>",
    "It calls .unwrap() on the JSON it was given",
    "It has /// docs with an example",
    "It uses ? to pass errors up"
   ],
   "answer": 1,
   "why": "Input you were handed can be wrong, so a library returns the error instead of panicking. unwrap is fine in tests, not on someone else's data."
  }
 ]
};
