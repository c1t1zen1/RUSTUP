// Checkpoint 2: review of Sessions 4 and 5. type: "mc" (pick one) or "text" (typed answer, any of `answers`, case and spaces ignored).
window.QUIZ = {
 "id": "s5-cp2",
 "title": "Checkpoint 2: Sessions 4 and 5",
 "pass": 70,
 "questions": [
  {
   "type": "text",
   "prompt": "What does this print?",
   "answers": [
    "2"
   ],
   "why": "entry finds the slot, or_insert(0) starts it at 0, and += 1 counts each word. beep appears twice. (Session 4)",
   "code": "let mut counts = HashMap::new();\nfor w in \"beep boop beep\".split_whitespace() {\n    *counts.entry(w).or_insert(0) += 1;\n}\nprintln!(\"{}\", counts[\"beep\"]);"
  },
  {
   "type": "text",
   "prompt": "What does this print?",
   "answers": [
    "[3, 2, 1]",
    "[3,2,1]"
   ],
   "why": "Comparing b to a, instead of a to b, reverses the order: largest first. (Session 4)",
   "code": "let mut v = vec![3, 1, 2];\nv.sort_by(|a, b| b.cmp(a));\nprintln!(\"{v:?}\");"
  },
  {
   "type": "mc",
   "prompt": "Which trait must a struct have to be compared with <code>==</code>?",
   "options": [
    "Debug",
    "Clone",
    "PartialEq",
    "Display"
   ],
   "answer": 2,
   "why": "== calls PartialEq::eq. #[derive(PartialEq)] writes it field by field. (Session 4)"
  },
  {
   "type": "mc",
   "prompt": "Will <code>show(vec![1, 2])</code> compile?",
   "options": [
    "Yes, it prints [1, 2]",
    "No: Vec doesn't implement Display",
    "No: generics can't take Vecs",
    "Yes, but it panics"
   ],
   "answer": 1,
   "why": "The bound says T: Display, and Vec only has Debug. That's E0277. Use {:?} and T: Debug instead. (Session 4)",
   "code": "fn show<T: std::fmt::Display>(x: T) {\n    println!(\"{x}\");\n}"
  },
  {
   "type": "text",
   "prompt": "What does this print?",
   "answers": [
    "18"
   ],
   "why": "filter keeps 3, 6 and 9, and sum adds them. (Session 4)",
   "code": "let s: i32 = (1..=10).filter(|n| n % 3 == 0).sum();\nprintln!(\"{s}\");"
  },
  {
   "type": "mc",
   "prompt": "<code>fn first&lt;'a&gt;(a: &amp;'a str, b: &amp;str) -&gt; &amp;'a str</code>. Which input may the result borrow from?",
   "options": [
    "a only",
    "b only",
    "Either",
    "Neither: it must be a new String"
   ],
   "answer": 0,
   "why": "Only a carries 'a, so the result is tied to a. The caller may drop b straight away. (Session 4)"
  },
  {
   "type": "mc",
   "prompt": "You need one Vec holding different types that all implement <code>Bot</code>. Which element type?",
   "options": [
    "impl Bot",
    "Box<dyn Bot>",
    "Rc<Bot>",
    "T where T: Bot"
   ],
   "answer": 1,
   "why": "A Vec holds one type. Box&lt;dyn Bot&gt; is one type that can point at any bot. (Session 4)"
  },
  {
   "type": "mc",
   "prompt": "Which of these is NOT a reason to use <code>Box</code>?",
   "options": [
    "A type that contains itself",
    "Holding different types behind dyn Trait",
    "Sharing one value between several owners",
    "Keeping a big value off the stack"
   ],
   "answer": 2,
   "why": "A Box has exactly one owner. Several owners is Rc (or Arc across threads). (Session 5)"
  },
  {
   "type": "mc",
   "prompt": "Eight threads must all change one <code>Vec</code>. Which type?",
   "options": [
    "Rc<RefCell<Vec<T>>>",
    "Arc<Mutex<Vec<T>>>",
    "Box<Vec<T>>",
    "&mut Vec<T>"
   ],
   "answer": 1,
   "why": "Many owners across threads is Arc; changing it safely is Mutex. Rc and RefCell aren't Send. (Session 5)"
  },
  {
   "type": "text",
   "prompt": "What does this print?",
   "answers": [
    "6"
   ],
   "why": "Each thread sends its number, the first tx is dropped so the loop ends, and 1 + 2 + 3 = 6 whatever order they arrive in. (Session 5)",
   "code": "let (tx, rx) = mpsc::channel();\nfor n in 1..=3 {\n    let tx = tx.clone();\n    thread::spawn(move || tx.send(n).unwrap());\n}\ndrop(tx);\nlet total: i32 = rx.iter().sum();\nprintln!(\"{total}\");"
  },
  {
   "type": "mc",
   "prompt": "What does <code>RefCell::try_borrow_mut</code> give you that <code>borrow_mut</code> doesn't?",
   "options": [
    "Faster access",
    "A Result instead of a panic when it's already borrowed",
    "Thread safety",
    "A copy of the value"
   ],
   "answer": 1,
   "why": "borrow_mut panics on a clash. try_borrow_mut returns Err, so you can handle it. (Session 5)"
  },
  {
   "type": "mc",
   "prompt": "Does this compile, and what does it print?",
   "options": [
    "Yes, 10",
    "No: E0373, it needs move",
    "No: readings was moved",
    "Yes, but the order is random"
   ],
   "answer": 0,
   "why": "Scoped threads are joined before scope returns, so they may borrow readings. 1 + 2 + 3 + 4 = 10. (Session 5)",
   "code": "let readings = vec![1, 2, 3, 4];\nlet (l, r) = readings.split_at(2);\nlet total = thread::scope(|s| {\n    let a = s.spawn(|| l.iter().sum::<i32>());\n    let b = s.spawn(|| r.iter().sum::<i32>());\n    a.join().unwrap() + b.join().unwrap()\n});\nprintln!(\"{total}\");"
  },
  {
   "type": "mc",
   "prompt": "When does a <code>MutexGuard</code> unlock?",
   "options": [
    "When you call unlock()",
    "When the guard is dropped",
    "After one second",
    "When the thread ends"
   ],
   "answer": 1,
   "why": "There's no unlock method. The guard unlocks when it goes out of scope, so keep it short. (Session 5)"
  },
  {
   "type": "mc",
   "prompt": "A server keeps 10,000 connections open, each mostly waiting. Threads or async?",
   "options": [
    "Threads: one per connection",
    "Async tasks on a runtime like tokio",
    "Neither: use a Mutex",
    "Scoped threads"
   ],
   "answer": 1,
   "why": "Async tasks are cheap and switch while waiting. Threads suit heavy computing on every core. (Session 5)"
  },
  {
   "type": "mc",
   "prompt": "What can a test in <code>tests/mine.rs</code> call?",
   "options": [
    "Anything in the crate",
    "Only pub items, like a user of the crate",
    "Only functions in main.rs",
    "Only #[test] functions"
   ],
   "answer": 1,
   "why": "Files in tests/ are integration tests: they use your crate from the outside. Unit tests in mod tests see private items. (Session 5)"
  }
 ]
};
