// Quiz 5: Many Robbies at Once. type: "mc" (pick one) or "text" (typed answer, any of `answers`, case and spaces ignored).
window.QUIZ = {
 "id": "s5",
 "title": "Quiz 5: Many Robbies at Once",
 "pass": 70,
 "questions": [
  {
   "type": "mc",
   "prompt": "Why won't this compile?",
   "options": [
    "char can't go in an enum",
    "A Route contains a whole Route, so it has no fixed size",
    "End needs a value",
    "Enums can't be recursive in Rust"
   ],
   "answer": 1,
   "why": "The fix is Step(char, Box&lt;Route&gt;): a Box is always one pointer wide. This is E0072.",
   "code": "enum Route {\n    Step(char, Route),\n    End,\n}"
  },
  {
   "type": "mc",
   "prompt": "What does <code>Rc::clone(&amp;map)</code> do?",
   "options": [
    "Copies the whole map",
    "Adds one more owner and bumps the count",
    "Moves the map",
    "Makes a weak reference"
   ],
   "answer": 1,
   "why": "Rc shares one value. clone adds an owner; the value is freed when the last owner goes."
  },
  {
   "type": "mc",
   "prompt": "What happens here?",
   "options": [
    "Prints [1, 2, 3]",
    "Compile error E0502",
    "It compiles, then panics: already borrowed",
    "Prints [1, 2]"
   ],
   "answer": 2,
   "why": "RefCell checks the borrow rules at run time. A borrow_mut while reading is still alive panics.",
   "code": "use std::cell::RefCell;\n\nfn main() {\n    let parts = RefCell::new(vec![1, 2]);\n    let reading = parts.borrow();\n    parts.borrow_mut().push(3);\n    println!(\"{reading:?}\");\n}"
  },
  {
   "type": "mc",
   "prompt": "This fails with E0373. What's the usual fix?",
   "options": [
    "Add move before the closure",
    "Make rooms mut",
    "Call join twice",
    "Use Rc"
   ],
   "answer": 0,
   "why": "The thread might outlive main's rooms, so it can't borrow it. move gives the thread ownership.",
   "code": "let rooms = vec![\"studio\", \"lobby\"];\nlet h = thread::spawn(|| println!(\"{rooms:?}\"));\nh.join().unwrap();"
  },
  {
   "type": "text",
   "prompt": "What does this print?",
   "answers": [
    "15"
   ],
   "why": "Three threads each add 5 under the lock, and every thread is joined before printing. Always 15.",
   "code": "let n = Arc::new(Mutex::new(0));\nlet mut hs = Vec::new();\nfor _ in 0..3 {\n    let n = Arc::clone(&n);\n    hs.push(thread::spawn(move || {\n        for _ in 0..5 {\n            *n.lock().unwrap() += 1;\n        }\n    }));\n}\nfor h in hs {\n    h.join().unwrap();\n}\nprintln!(\"{}\", n.lock().unwrap());"
  },
  {
   "type": "mc",
   "prompt": "Every thread has finished, but <code>for msg in rx</code> never ends. Why?",
   "options": [
    "Channels hold only one message",
    "A sender is still alive, usually the first tx in main",
    "rx needs to be mut",
    "The threads weren't joined"
   ],
   "answer": 1,
   "why": "The loop ends when every sender has been dropped. drop(tx) after the spawning loop."
  },
  {
   "type": "mc",
   "prompt": "<code>thread::spawn(move || println!(\"{copy}\"))</code> where <code>copy</code> is an <code>Rc&lt;String&gt;</code> fails with E0277. What do you swap in?",
   "options": [
    "Box",
    "RefCell",
    "Arc",
    "Cell"
   ],
   "answer": 2,
   "why": "Rc's counter isn't atomic, so Rc isn't Send. Arc is the thread-safe version."
  },
  {
   "type": "mc",
   "prompt": "A type is <code>Send</code>. What does that promise?",
   "options": [
    "It can be sent over a network",
    "A value of it can be moved to another thread safely",
    "It can be shared by reference between threads",
    "It implements Clone"
   ],
   "answer": 1,
   "why": "Send: can move to another thread. Sync: can be shared by &amp; between threads."
  },
  {
   "type": "text",
   "prompt": "Fill the blank so the threads may borrow <code>readings</code> without move: <code>let total = thread::______(|s| { ... });</code>",
   "answers": [
    "scope"
   ],
   "why": "Scoped threads are all joined before thread::scope returns, so borrowing local data is safe."
  },
  {
   "type": "mc",
   "prompt": "What does this print?",
   "options": [
    "beep!, then plan made",
    "plan made, then beep!",
    "Only plan made",
    "Nothing: it doesn't compile"
   ],
   "answer": 1,
   "why": "Calling an async fn only makes a future. Nothing runs until .await, which comes after the first println.",
   "code": "async fn beep() { println!(\"beep!\"); }\n\n#[tokio::main]\nasync fn main() {\n    let later = beep();\n    println!(\"plan made\");\n    later.await;\n}"
  },
  {
   "type": "mc",
   "prompt": "Which attribute means a module is only compiled for <code>cargo test</code>?",
   "options": [
    "#[test]",
    "#[cfg(test)]",
    "#[derive(Test)]",
    "#[should_panic]"
   ],
   "answer": 1,
   "why": "#[cfg(test)] goes on the mod tests block. #[test] goes on each test function."
  },
  {
   "type": "mc",
   "prompt": "A JSON config is missing a field your <code>#[derive(Deserialize)]</code> struct needs. What does <code>serde_json::from_str</code> do?",
   "options": [
    "Fills in zero",
    "Panics",
    "Returns an Err saying the field is missing",
    "Skips the struct"
   ],
   "answer": 2,
   "why": "serde never guesses. You get Err(\"missing field ...\"), unless you mark the field #[serde(default)]."
  }
 ]
};
