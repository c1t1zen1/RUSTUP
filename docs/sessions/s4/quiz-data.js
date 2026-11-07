// Quiz 4: Teaching Robbie New Tricks. type: "mc" (pick one) or "text" (typed answer, any of `answers`, case and spaces ignored).
window.QUIZ = {
 "id": "s4",
 "title": "Quiz 4: Teaching Robbie New Tricks",
 "pass": 70,
 "questions": [
  {
   "type": "mc",
   "prompt": "What does <code>parts.get(10)</code> return when <code>parts</code> has 3 items?",
   "options": [
    "It panics",
    "None",
    "An empty String",
    "The last item"
   ],
   "answer": 1,
   "why": "get returns an Option, so a missing index is None. parts[10] is the one that panics."
  },
  {
   "type": "mc",
   "prompt": "What happens here?",
   "options": [
    "Prints 87",
    "Error E0502: can't push while first borrows readings",
    "Prints 13",
    "It panics at runtime"
   ],
   "answer": 1,
   "why": "push needs &amp;mut readings, but first is a shared borrow that's still used afterwards. Push may move the items, so Rust refuses.",
   "code": "let mut readings = vec![87, 42];\nlet first = &readings[0];\nreadings.push(13);\nprintln!(\"{first}\");"
  },
  {
   "type": "text",
   "prompt": "Fill the blank to count each word: <code>*counts.entry(word).______(0) += 1;</code>",
   "answers": [
    "or_insert"
   ],
   "why": "entry finds the slot, or_insert(0) fills it if it's new and gives back a &amp;mut to the count."
  },
  {
   "type": "mc",
   "prompt": "A trait has <code>fn name(&amp;self) -&gt; String;</code> and <code>fn greet(&amp;self) -&gt; String { ... }</code> with a body. Which must every <code>impl</code> write?",
   "options": [
    "Both",
    "Only name",
    "Only greet",
    "Neither"
   ],
   "answer": 1,
   "why": "greet has a default body, so it's optional. name has none, so leaving it out is E0046."
  },
  {
   "type": "mc",
   "prompt": "Will this compile?",
   "options": [
    "Yes, it prints 9",
    "No: T has no bound that allows >",
    "No: slices can't be generic",
    "Yes, but it panics"
   ],
   "answer": 1,
   "why": "The body compares two T values, so T needs PartialOrd. Without the bound it's E0369.",
   "code": "fn largest<T: Copy>(items: &[T]) -> T {\n    let mut best = items[0];\n    for &i in items {\n        if i > best {\n            best = i;\n        }\n    }\n    best\n}"
  },
  {
   "type": "mc",
   "prompt": "<code>fn show(x: &amp;impl Speak)</code> means the same as...",
   "options": [
    "fn show(x: &dyn Speak)",
    "fn show<T: Speak>(x: &T)",
    "fn show(x: Speak)",
    "fn show(x: &Speak)"
   ],
   "answer": 1,
   "why": "impl Trait in an argument is shorthand for a generic with that bound. It's decided at compile time."
  },
  {
   "type": "mc",
   "prompt": "You need one <code>Vec</code> holding a robot, a drone and a vacuum. Which type?",
   "options": [
    "Vec<Bot>",
    "Vec<impl Bot>",
    "Vec<Box<dyn Bot>>",
    "Vec<T: Bot>"
   ],
   "answer": 2,
   "why": "A Vec holds one type. Box&lt;dyn Bot&gt; is one type that can point at any bot."
  },
  {
   "type": "mc",
   "prompt": "Which derive lets you use a struct as a HashMap key?",
   "options": [
    "Debug",
    "Clone",
    "Hash, with PartialEq and Eq",
    "Display"
   ],
   "answer": 2,
   "why": "Keys must be hashed and compared, so they need Hash and Eq (which needs PartialEq)."
  },
  {
   "type": "text",
   "prompt": "What does this print?",
   "answers": [
    "5"
   ],
   "why": "The closure captures squeaks and changes it, so it's FnMut. 4 / 2 + 6 / 2 = 5.",
   "code": "let mut squeaks = 0;\nlet mut walk = |steps: u32| squeaks += steps / 2;\nwalk(4);\nwalk(6);\nprintln!(\"{squeaks}\");"
  },
  {
   "type": "text",
   "prompt": "What does this print?",
   "answers": [
    "[2, 6, 10]",
    "[2,6,10]"
   ],
   "why": "filter keeps the odd numbers 1, 3 and 5, then map doubles them.",
   "code": "let v: Vec<i32> = (1..=6).filter(|n| n % 2 == 1).map(|n| n * 2).collect();\nprintln!(\"{v:?}\");"
  },
  {
   "type": "mc",
   "prompt": "How many times does <code>check</code> run?",
   "options": [
    "0",
    "2",
    "5",
    "It doesn't compile"
   ],
   "answer": 1,
   "why": "Iterators are lazy. take(2) only asks for two items, so the closure runs twice.",
   "code": "let parts = [\"a\", \"b\", \"c\", \"d\", \"e\"];\nlet n = parts.iter().map(|p| check(p)).take(2).count();"
  },
  {
   "type": "mc",
   "prompt": "Why does this fail to compile?",
   "options": [
    "&str can't be returned",
    "The compiler can't tell whether the result borrows from a or b, so it needs a lifetime",
    "if needs a semicolon",
    "a and b must be String"
   ],
   "answer": 1,
   "why": "Two references in and one out: you say which inputs the answer borrows from with &lt;'a&gt;. This is E0106.",
   "code": "fn longer(a: &str, b: &str) -> &str {\n    if a.len() >= b.len() { a } else { b }\n}"
  }
 ]
};
