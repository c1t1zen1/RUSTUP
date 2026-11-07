# Homework 5: Swarm Sim

The studio floor is filthy and the swarm is ready. A config file says how dirty each tile is and where each cleaning bot starts and walks. Your program reads it with serde, checks it, sends every bot out on its own thread over one shared floor, and writes a report back out as JSON.

The project is split into modules:

| File | What's in it | Your job |
| --- | --- | --- |
| `src/config.rs` | `Config`, `BotPlan` and `SimError` | The serde derives, the `Display` messages, and `Config::from_json` with every check |
| `src/sim.rs` | `BotReport`, `Report` and `run` | The `Serialize` derives, `Report::to_json`, and `run`: one thread per bot, one `Arc<Mutex<..>>` floor |
| `src/lib.rs` | `simulate_json` | Nothing, it's done |
| `src/main.rs` | Reads `swarm.json`, prints a summary, writes `report.json` | Nothing, it's done |
| `tests/mine.rs` | One example test | Write at least three more of your own |

Every rule is in the doc comment above what you're writing. The hidden tests check exactly those rules.

```
cargo test                    # 8 visible tests, plus yours
cargo clippy -- -D warnings   # no warnings allowed
cargo fmt --check             # formatted the standard way
cargo run                     # three bots, one dirty floor
cargo run -- my-floor.json    # or your own config
```

The config looks like this. `start` is `[x, y]`, counted from 0 at the top left, and a route is made of the letters N, E, S and W:

```json
{
  "floor": [[3, 1, 0], [0, 2, 5]],
  "bots": [{ "name": "Robbie", "start": [0, 0], "route": "EES" }]
}
```

| Graded on | Points |
| --- | --- |
| Builds | 2 |
| Visible tests pass | 2 |
| Your own tests: at least three more, all passing, each checking a rule the visible tests don't | 1 |
| Hidden tests pass (the order of the checks, empty rows, tiles that never go below zero, crowded floors that give the same totals every run, and every error message) | 3 |
| clippy clean | 1 |
| fmt clean | 1 |

Hints: `#[derive(Serialize, Deserialize)]` is most of the serde work, and a `(usize, usize)` reads and writes as a two-number JSON array. `map_err` turns serde_json's error into yours, and `?` does the rest. Hold the floor's lock only while you change one tile, never for a whole walk, or the bots take turns instead of working together. Collect every `JoinHandle` before joining any. And sort the bot reports by name at the end, because threads finish in whatever order they like.
