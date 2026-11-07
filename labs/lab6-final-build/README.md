# Lab 6: the Final Build Lab

Robbie is 95% rust and has one job left before he retires to a nice dry shelf: ship. Today you'll write the kind of code that sits at the edges of a real crate. A macro, a little `unsafe` with a safe wrapper around it, two calls into the C library, a benchmark, and the paperwork `cargo publish` asks for.

`vowels_chars`, `vowels_bytes` and `status` are written for you in `src/lib.rs`. Fill in the rest, in order. Each step has its own tests.

| Step | What you write | Check it |
| --- | --- | --- |
| 1 | `beep!` with three arms, and `loudest!` calling itself | `cargo test step1` |
| 2 | `split_halves`: two `&mut` halves from one slice, with raw pointers | `cargo test step2` |
| 3 | an `unsafe extern "C"` block, then safe `c_abs` and `c_strlen` | `cargo test step3` |
| 4 | `time_it`, then run the benchmark in `examples/bench.rs` | `cargo test step4` |
| 5 | the `[package]` fields in `Cargo.toml`, and a doc example on every `pub fn` | `cargo test step5` |

When all nine tests pass, run everything the grader will run:

```
cargo test                          # the steps, plus every example in your docs
cargo run --release --example bench # which vowel counter wins?
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo doc --open                    # your docs, as a website
cargo publish --dry-run             # packs the crate and checks it, uploads nothing
```

Rules for today:

- Every `unsafe` block gets a `// SAFETY:` comment saying why it's fine. If you can't write the comment, you can't write the block.
- `unsafe` stays inside the function. Callers of `split_halves`, `c_abs` and `c_strlen` never write `unsafe` themselves.
- C's `abs(INT_MIN)` is undefined behaviour. Your `c_abs` must never pass it `i32::MIN`.
- Benchmark in release mode only. A debug build is ten times slower and tells you nothing.
- Never actually run `cargo publish` without `--dry-run` today. Real publishing is forever: you can yank a version, but never delete it.

Finished early? Add a third arm to `beep!` that takes a list, so `beep!["beep", "boop", "bzzt"]` gives `"beep boop bzzt"`. Then try `split_halves` without `unsafe`, using `slice::split_at_mut`. That's the safe function the standard library wrote so you don't have to.
