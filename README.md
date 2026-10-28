# 🦀 Rustvember

Three challengers, one month, three Rust CLI task managers.

This repo is a shared learning challenge for November 2026. Each of us builds
our **own** task manager from scratch in our own directory. We share the repo
so we can read each other's code, review PRs, compare approaches, and keep
each other accountable — but nobody copies, and every project must compile
and run independently.

## Participants

| Name      | Directory   | Binary name       | Background |
| --------- | ----------- | ----------------- | ---------- |
| @kris     | `kris/`     | `<your cli name>` | TS / JS    |
| @michael  | `michael/`  | `<your cli name>` | Swift      |
| @emmanuel | `emmanuel/` | `<your cli name>` | TS / JS    |

## Goal

By the end of the month, each of us should be able to open an unfamiliar Rust
codebase (e.g. `ripgrep`, `bat`, `serde_json`) and confidently read it. We should also be okay to go further and work with crates, and build full backends if we wanted to.
The project is a vehicle for learning:

- Ownership, borrowing, and lifetimes
- Structs, enums, and exhaustive `match`
- `Option`, `Result`, and the `?` operator
- Traits, generics, and dynamic dispatch (`Box<dyn Trait>`)
- Iterators and closures
- Modules, Cargo, and the crate ecosystem
- Unit testing with `cargo test`

## Repository layout

```text
.
├── README.md
├── CHALLENGE.md        # weekly milestones and feature spec (below, in detail)
├── alice/              # Alice's project — a standalone Cargo crate
│   ├── Cargo.toml
│   └── src/
├── bob/
│   ├── Cargo.toml
│   └── src/
└── charlie/
    ├── Cargo.toml
    └── src/
```

Each directory is an independent Cargo project. There is **no** root
workspace on purpose — everyone owns their own `Cargo.toml`, dependencies, and
toolchain decisions.

```bash
cd michael
cargo run -- add "Buy milk"
cargo test
```

## The challenge

### Week 1 — Fundamentals (std only, no external crates)

Read [The Rust Book](https://doc.rust-lang.org/book/) ch. 1–10 alongside.

- [ ] `add <title>` — create a task
- [ ] `list` — print all tasks
- [ ] `done <id>` — mark a task complete
- [ ] `rm <id>` — delete a task
- [ ] Auto-incrementing IDs
- [ ] Persist tasks to a plain-text file (hand-rolled format)
- [ ] Parse `std::env::args()` manually into a `Command` enum

### Week 2 — Idiomatic Rust (crates + refactor)

- [ ] Replace manual arg parsing with `clap` (derive API)
- [ ] Replace text format with JSON via `serde` / `serde_json`
- [ ] Custom error enum (`thiserror` or hand-rolled `impl Error`)
- [ ] `list --status <todo|done>` filtering
- [ ] `edit <id> <title>`
- [ ] `clear` completed tasks
- [ ] A `Storage` trait with `JsonStorage` and `MemoryStorage` implementations
- [ ] Unit tests against `MemoryStorage`
- [ ] Zero warnings from `cargo clippy`

### Week 3 — Stretch (pick at least 2)

- [ ] `SqliteStorage` via `rusqlite` behind the same `Storage` trait
- [ ] Due dates with `chrono` and a custom `FromStr`
- [ ] Tags (`--tag`) using `HashSet<String>`
- [ ] `search <query>`
- [ ] `list --sort <due|priority|created>`
- [ ] `undo`
- [ ] Subtasks
- [ ] Colored output
- [ ] Export to CSV / Markdown via `Box<dyn Exporter>`
- [ ] Interactive REPL mode

### Week 4 — Read real code

- [ ] Read Rust Book ch. 15–16 (smart pointers, concurrency)
- [ ] Each person picks a real crate, reads it, and writes a short
      `NOTES.md` in their directory on what patterns they recognized
- [ ] Review each other's final projects

## Ground rules

1. **Your directory, your code.** Only commit to your own folder. Shared
   files at the root (this README, `CHALLENGE.md`, CI) go through a PR.
2. **Open PRs, even to yourself.** Push features as PRs so the other two can
   read and comment. Reviews are for learning, not gatekeeping — approve
   freely, ask questions liberally.
3. **No copy-paste between projects.** Reading each other's code is
   encouraged; lifting it is not. If you learn a trick, re-implement it.
4. **Lean on the compiler and Clippy** before asking a human or an LLM.
   Read the full error message first.
5. **Commit something every day**, even if it's a failing test.
6. **Weekly sync** (Mondays): demo, show your ugliest borrow-checker fight,
   compare approaches to the same feature.
7. It's a learning project. Heavily encourage **you to write code by hand**. You can use AI to help you triage and understand issues or further understanding how something would work, but writing by hand is still the best way to learn the new language
8. You can use **any resource to learn** - YT tutorials, the rust book etc. There'll be a RESOURCES.md file at the root that you can contribute to, to share resources you find

## Conventions

- Rust edition 2021, stable toolchain (`rustup update stable`)
- Format with `cargo fmt` before every commit
- `cargo clippy -- -D warnings` should pass from Week 2 onward
- Branch naming: `<name>/<feature>`, e.g. `bob/json-storage`
- Commit messages: imperative mood, e.g. `Add Storage trait and JSON impl`

## CI

GitHub Actions runs `cargo fmt --check`, `cargo clippy`, and `cargo test` in
each participant's directory on every PR. A failing check in _your_ folder
only blocks _your_ PR.

## Resources

- [The Rust Programming Language (The Book)](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) — small exercises
- [Clippy lint list](https://rust-lang.github.io/rust-clippy/master/)
- [docs.rs](https://docs.rs) — read the docs of every crate you add
- Crates: [`clap`](https://docs.rs/clap), [`serde`](https://serde.rs),
  [`thiserror`](https://docs.rs/thiserror), [`rusqlite`](https://docs.rs/rusqlite),
  [`chrono`](https://docs.rs/chrono)

## Progress

| Milestone | @kris | @michael | @emmanuel |
| --------- | :---: | :------: | :-------: |
| Week 1    |  ⬜   |    ⬜    |    ⬜     |
| Week 2    |  ⬜   |    ⬜    |    ⬜     |
| Week 3    |  ⬜   |    ⬜    |    ⬜     |
| Week 4    |  ⬜   |    ⬜    |    ⬜     |

---

### After November — SQLite (optional)

Swap `JsonStorage` for `SqliteStorage` without touching command logic.
See `CHALLENGE.md`.

---

_Started November 1, 2026. Ends when the borrow checker stops winning._
