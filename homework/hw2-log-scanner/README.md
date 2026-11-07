# Homework 2: Log Scanner

Robbie keeps a log. It's full of squeaks, coffee incidents and, for some reason, a wifi password. Write the functions that read it.

The menu in `src/main.rs` is done. Your job is `src/lib.rs`: replace each `todo!()` with a real body. Every function **borrows** its input, so the log is never moved or copied.

```
cargo test                    # 8 visible tests
cargo clippy -- -D warnings   # no warnings allowed
cargo fmt --check             # formatted the standard way
cargo run                     # see Robbie's log report
```

| Graded on | Points |
| --- | --- |
| Builds | 2 |
| Visible tests pass | 3 |
| Hidden tests pass (empty logs, ties, secrets that repeat or aren't there) | 3 |
| clippy clean | 1 |
| fmt clean | 1 |

Hints: `.lines()` walks a `&str` line by line, `.starts_with()` checks a prefix, `.replace()` builds a new String, and `"*".repeat(n)` makes n stars.
