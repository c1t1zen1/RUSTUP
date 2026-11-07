# Lab 5: Swarm Mode

Robbie has made friends. Five of them, all Robbie v11, all slightly rusty, and the studio wants every room scanned before the rain comes in. One Robbie doing five rooms is slow. Five Robbies doing one room each is swarm mode, and that means threads.

`dust_in` and the `Job` struct are written for you in `src/lib.rs`. Fill in the five `todo!()`s, in order, adding the imports you need as you go. Each step has its own tests.

| Step | What you write | Check it |
| --- | --- | --- |
| 1 | `scan_rooms`: `thread::spawn` with a `move` closure, then `join` | `cargo test step1` |
| 2 | `total_dust`: an `mpsc` channel, one sender per thread | `cargo test step2` |
| 3 | `count_squeaks`: one counter shared with `Arc<Mutex<u32>>` | `cargo test step3` |
| 4 | `scoped_sum`: `thread::scope` and `chunks`, no `move` of the data needed | `cargo test step4` |
| 5 | `run_jobs`: a worker pool sharing one job queue | `cargo test step5` |

When all nine tests pass:

```
cargo run
Swarm mode: five Robbies, five rooms.

  studio       20 dust
  workshop     51 dust
  ...
Squeaks from 5 bots walking 10 steps: 25

Worker pool, 3 workers:
  job 1: studio: 20
  ...
```

Rules for today:

- Collect every `JoinHandle` before you join any of them. Joining inside the same loop that spawns runs the rooms one at a time.
- In step 2, the loop over `rx` only ends when every sender is gone, including the one you made first. If your test hangs, that's why.
- Try swapping `Arc` for `Rc` in step 3, once, on purpose. Read the E0277 error. That's `Send` doing its job.
- In step 5, a `Receiver` can't be shared, but an `Arc<Mutex<Receiver<Job>>>` can. Lock it just long enough to take one job.
- `cargo clippy -- -D warnings` must be clean before you call it done.

Finished early? Give `run_jobs` a way to say which worker did each job, and print it in `main`. Run it a few times. The order changes, and the sorted reports don't.
