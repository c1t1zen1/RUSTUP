# Homework 4: Bot Garage

Robbie isn't the only robot in the studio any more. There's a camera drone called Zippy, a vacuum called Dusty, and a garage with exactly enough power for some of them. Your job is to teach them all one shared trait, `Bot`, and build a garage that holds any mix of them without blowing a fuse.

The project is split into modules:

| File | What's in it | Your job |
| --- | --- | --- |
| `src/bots.rs` | The `Bot` trait, and `Robbie`, `Drone` and `Vacuum` | Implement `Bot` for all three, including Robbie's own `label` |
| `src/garage.rs` | `GarageError` and `Garage` | The `Display` messages and seven `Garage` methods |
| `src/lib.rs` | Two generic functions | `largest` and `hungriest` |
| `src/main.rs` | Parks five bots and prints a report | Nothing, it's done |

Every rule is in the doc comment above what you're writing. The hidden tests check exactly those rules.

```
cargo test                    # 8 visible tests
cargo clippy -- -D warnings   # no warnings allowed
cargo fmt --check             # formatted the standard way
cargo run                     # five bots arrive, two get turned away
```

| Graded on | Points |
| --- | --- |
| Builds | 2 |
| Visible tests pass | 3 |
| Hidden tests pass (the squeak at exactly 50%, names in any case, the exact budget, refused bots changing nothing, unparking, empty garages, ties) | 3 |
| clippy clean | 1 |
| fmt clean | 1 |

Hints: a trait's default method is used unless you write your own, so `Drone` and `Vacuum` don't need `label`. `Box<dyn Bot>` lets one `Vec` hold different types. `iter().map(...).sum()` adds things up, `eq_ignore_ascii_case` compares names, `position` finds an index, and the `entry` API from the lab counts kinds.
