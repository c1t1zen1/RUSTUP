// Quiz 3: Building Blocks. type: "mc" (pick one) or "text" (typed answer, any of `answers`, case and spaces ignored).
window.QUIZ = {
 "id": "s3",
 "title": "Quiz 3: Building Blocks",
 "pass": 70,
 "questions": [
  {
   "type": "mc",
   "prompt": "Which keyword defines a type with named fields that all exist at once?",
   "options": [
    "struct",
    "enum",
    "impl",
    "mod"
   ],
   "answer": 0,
   "why": "A struct is AND: every field is there. An enum is OR: exactly one variant."
  },
  {
   "type": "mc",
   "prompt": "Which method signature lets <code>charge</code> change Robbie's battery?",
   "options": [
    "fn charge(self)",
    "fn charge(&self)",
    "fn charge(&mut self)",
    "fn charge()"
   ],
   "answer": 2,
   "why": "&amp;mut self borrows Robbie mutably. &amp;self can only read, self would use him up, and no self means it isn't a method at all."
  },
  {
   "type": "mc",
   "prompt": "<code>impl Robot</code> has <code>fn new(name: &amp;str) -&gt; Robot</code>. How do you call it?",
   "options": [
    "Robot.new(\"Robbie\")",
    "Robot::new(\"Robbie\")",
    "new Robot(\"Robbie\")",
    "robot->new(\"Robbie\")"
   ],
   "answer": 1,
   "why": "No self parameter means it's an associated function, called on the type with ::. A dot is for methods on a value."
  },
  {
   "type": "mc",
   "prompt": "What happens with this code?",
   "options": [
    "It prints stop",
    "It compiles, then panics on Amber",
    "It doesn't compile: Amber isn't covered",
    "Amber is quietly ignored"
   ],
   "answer": 2,
   "why": "match must be exhaustive. Missing a variant is error E0004, caught before the program ever runs.",
   "code": "enum Light { Green, Amber, Red }\n\nfn action(l: Light) -> &'static str {\n    match l {\n        Light::Green => \"go\",\n        Light::Red => \"stop\",\n    }\n}"
  },
  {
   "type": "text",
   "prompt": "What does this print?",
   "answers": [
    "7"
   ],
   "why": "unwrap_or gives the value inside Some, or the default when it's None. Here it's None, so 7.",
   "code": "let shelf: Option<u32> = None;\nprintln!(\"{}\", shelf.unwrap_or(7));"
  },
  {
   "type": "mc",
   "prompt": "How does Rust say \"there might not be a value here\"?",
   "options": [
    "null",
    "Option<T>",
    "Result<T, E>",
    "Return -1"
   ],
   "answer": 1,
   "why": "Option&lt;T&gt; is Some(value) or None. Rust has no null, so a missing value can't sneak in unannounced."
  },
  {
   "type": "mc",
   "prompt": "What does <code>\"forty\".parse::&lt;u32&gt;()</code> give you?",
   "options": [
    "0",
    "None",
    "Err(...) with a ParseIntError",
    "It panics"
   ],
   "answer": 2,
   "why": "parse returns a Result. Text that isn't a number gives Err, and nothing panics unless you unwrap it."
  },
  {
   "type": "mc",
   "prompt": "Inside a function that returns <code>Result</code>, what does <code>?</code> do when the value is <code>Err</code>?",
   "options": [
    "Panics",
    "Ignores it and carries on",
    "Returns that Err from the function right away",
    "Turns it into None"
   ],
   "answer": 2,
   "why": "? is the hot potato: Ok gets unwrapped, Err gets returned to the caller immediately."
  },
  {
   "type": "mc",
   "prompt": "Will this compile?",
   "options": [
    "Yes",
    "No: main returns (), so ? has nowhere to send an error"
   ],
   "answer": 1,
   "why": "? only works in a function that returns Result or Option. This is error E0277. Make main return Result, or use match.",
   "code": "fn main() {\n    let n: u32 = \"5\".parse()?;\n    println!(\"{n}\");\n}"
  },
  {
   "type": "mc",
   "prompt": "Given <code>enum Command { Forward(u32), Say(String), Charge }</code>, which line makes a value?",
   "options": [
    "Command::Forward(3)",
    "Command.Forward = 3",
    "new Command::Forward(3)",
    "Command::Forward { 3 }"
   ],
   "answer": 0,
   "why": "A tuple-like variant is built like a function call: Command::Forward(3)."
  },
  {
   "type": "text",
   "prompt": "Which keyword makes a struct field usable from outside its module?",
   "answers": [
    "pub"
   ],
   "why": "Everything is private by default. pub on the struct doesn't open its fields: each field needs its own pub."
  },
  {
   "type": "mc",
   "prompt": "<code>if let Some(p) = part { ... }</code> is the best fit when...",
   "options": [
    "you need to handle every variant",
    "you only care about one pattern",
    "the value is a number",
    "you want to loop until it's empty"
   ],
   "answer": 1,
   "why": "if let is a one-armed match. For every variant use match; for looping, while let."
  }
 ]
};
