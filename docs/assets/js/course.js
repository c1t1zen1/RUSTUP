// RUSTUP course map: one source of truth for every page.
// rust: 0 = factory fresh, 5 = falling apart. rustPct is how much of Robbie is rust that session:
// 0, 15, 32, 50, 72, 95. By Session 6 his armor has rusted off and only a cracked chrome chest plate is left.
window.RUSTUP = {
  title: "RUSTUP",
  tagline: "A Rust master class with Robbie",
  sessions: [
    {
      id: "s1", n: 1, week: 1, day: "Mon", rust: 0, rustPct: 0, ready: true,
      theme: "Boot Sequence",
      blurb: "Install Rust, meet cargo, and write programs with variables, types, functions and loops.",
      topics: ["Why Rust", "rustup and cargo", "Variables and mutability", "Scalar and compound types", "Functions", "Expressions vs statements", "if, loop, while, for", "Shadowing", "rustfmt and clippy"],
      lab: "Robbie's Greeting",
      homework: "Units Converter",
      quiz: "Quiz 1"
    },
    {
      id: "s2", n: 2, week: 1, day: "Thu", rust: 1, rustPct: 15, ready: true,
      theme: "Who Owns the Wrench?",
      blurb: "Ownership, moves, borrowing and the borrow checker: the idea that makes Rust Rust.",
      topics: ["Stack vs heap", "Ownership and move", "Copy vs Clone", "Borrowing", "Slices", "String vs &str", "Lifetime basics"],
      lab: "Fix the Borrow Checker", homework: "Log Scanner", quiz: "Quiz 2"
    },
    {
      id: "s3", n: 3, week: 2, day: "Mon", rust: 2, rustPct: 32, ready: true,
      theme: "Building Blocks",
      blurb: "Structs, enums, match, Option and Result, error handling with ?, and modules.",
      topics: ["Structs and impl", "Enums with data", "match and patterns", "if let and let else", "Option and Result", "The ? operator", "Modules and privacy"],
      lab: "Robbie's Command Parser", homework: "Command Language", quiz: "Quiz 3 + Checkpoint 1"
    },
    {
      id: "s4", n: 4, week: 2, day: "Thu", rust: 3, rustPct: 50, ready: true,
      theme: "Teaching Robbie New Tricks",
      blurb: "Collections, traits, generics, closures and iterators.",
      topics: ["Vec and HashMap", "Traits", "Generics", "Closures", "Iterators", "Lifetimes in depth"],
      lab: "Mission Planner", homework: "Bot Garage", quiz: "Quiz 4"
    },
    {
      id: "s5", n: 5, week: 3, day: "Mon", rust: 4, rustPct: 72, ready: true,
      theme: "Many Robbies at Once",
      blurb: "Smart pointers, threads, channels, async, testing and serde.",
      topics: ["Box, Rc, RefCell, Arc", "Threads and channels", "Mutex, Send and Sync", "async with tokio", "Testing", "serde and JSON"],
      lab: "Swarm Mode", homework: "Swarm Sim", quiz: "Quiz 5 + Checkpoint 2"
    },
    {
      id: "s6", n: 6, week: 3, day: "Thu", rust: 5, rustPct: 95, ready: true,
      theme: "Ship It",
      blurb: "Macros, unsafe, FFI with C, performance, WebAssembly, then the final project.",
      topics: ["macro_rules!", "unsafe", "FFI with C", "Benchmarks", "WebAssembly", "Publishing"],
      lab: "Final Build Lab", homework: "Final Project", quiz: "Quiz 6 + Exam day"
    }
  ]
};
