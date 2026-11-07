# Lab 2: Fix the Borrow Checker

Seven tiny programs, seven compile errors. Each one breaks a different ownership or borrowing rule. Your job: make each one compile and print the right thing, following the goal written at the top of the file.

```
cargo run --bin puzzle1
```

Read the compiler's error first. It names the rule you broke and usually points at the fix.

| Puzzle | Rule it breaks |
| --- | --- |
| 1 The Name Tag | Using a value after it was moved |
| 2 The Shouting Function | A function taking ownership when it only needs to look |
| 3 The Recharge Station | Changing something through a shared `&` borrow |
| 4 The Parts List | A shared borrow alive while the vector changes |
| 5 Two Wrenches, One Bolt | Two `&mut` borrows at once |
| 6 The Dangling Antenna | Returning a reference to something that's about to be dropped |
| 7 The First Word | `&String` vs `&str`, and returning a slice |

When all seven run, `cargo clippy --bins` should be clean too.
