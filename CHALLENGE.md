# The Challenge

Build a CLI task manager in Rust over the four weeks of November 2026.
This document is the shared spec. Each participant implements it
independently in their own directory.

The point is not the task manager. The point is to be forced through every
core Rust concept until an unfamiliar Rust codebase stops looking foreign.

## Data model

A suggested starting point. Change field names and types freely, but the
shape should stay roughly this.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Todo,
    Done,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub status: Status,
    pub priority: Priority,
    pub created_at: u64, // unix seconds; swap for chrono later
}

pub enum Command {
    Add { title: String, priority: Priority },
    List { status: Option<Status> },
    Done(u32),
    Edit { id: u32, title: String },
    Remove(u32),
    Clear,
}
```

## Suggested module layout

```text
src/
├── main.rs       # parse args, call run(), print errors, set exit code
├── cli.rs        # arg parsing -> Command (manual in W1, clap in W2)
├── task.rs       # Task, Status, Priority, Display / FromStr impls
├── store.rs      # Storage trait + implementations
├── error.rs      # TodoError enum
└── commands.rs   # one function per Command, operates on &mut Vec<Task>
```

Start in a single `main.rs` if you like. Splitting into modules is itself a
Week 1 learning task.

---

## Week 1 — Fundamentals

**Constraint: `std` only. No external crates.**

**Read:** The Rust Book chapters 1–10. Rustlings sections through `traits`.

### Features

| Feature              | Example                 | Done |
| -------------------- | ----------------------- | :--: |
| Add a task           | `todo add "Buy milk"`   |  ⬜  |
| List tasks           | `todo list`             |  ⬜  |
| Complete a task      | `todo done 3`           |  ⬜  |
| Remove a task        | `todo rm 3`             |  ⬜  |
| Auto-incrementing ID | —                       |  ⬜  |
| Persist to text file | automatic, every run    |  ⬜  |
| Helpful usage errors | `todo done abc` → error |  ⬜  |

### Concepts you must touch

- `String` vs `&str` — when to own, when to borrow
- `Vec<Task>` with `iter()`, `iter_mut()`, `position()`, `remove()`
- `enum` + exhaustive `match` for `Command` and `Status`
- `Option<&Task>` / `Option<&mut Task>` from lookups
- `Result<T, E>` and the `?` operator for file I/O and parsing
- `impl Display for Task` and `impl FromStr for Task` for your text format
- `std::fs::read_to_string`, `std::fs::write`
- `str::parse::<u32>()`, `split`, `lines`, `trim`
- Splitting code into modules with `mod` and `pub`

### Definition of done

- All seven features work end to end
- Tasks survive between runs
- A malformed line in the data file produces an error message, not a panic
- No `unwrap()` outside of `main` (and ideally not there either)

---

## Week 2 — Idiomatic Rust

**Constraint: refactor, don't rewrite. Keep Week 1 behavior working.**

**Read:** Rust Book chapters 11–14 (testing, CLI project, closures and
iterators, Cargo). Skim the `clap` derive tutorial and the `serde` book.

### Features

| Feature            | Example                           | Done |
| ------------------ | --------------------------------- | :--: |
| `clap` arg parsing | `todo add -p high "Buy milk"`     |  ⬜  |
| JSON storage       | — (`serde_json`)                  |  ⬜  |
| Custom error type  | — (`TodoError` enum)              |  ⬜  |
| Filter by status   | `todo list --status done`         |  ⬜  |
| Edit title         | `todo edit 3 "Buy oat milk"`      |  ⬜  |
| Clear completed    | `todo clear`                      |  ⬜  |
| `Storage` trait    | `JsonStorage` + `MemoryStorage`   |  ⬜  |
| Unit tests         | `cargo test` against memory store |  ⬜  |
| Clippy clean       | `cargo clippy -- -D warnings`     |  ⬜  |

### The `Storage` trait

```rust
pub trait Storage {
    fn load(&self) -> Result<Vec<Task>, TodoError>;
    fn save(&self, tasks: &[Task]) -> Result<(), TodoError>;
}
```

Your command functions should take `&mut Vec<Task>` and know nothing about
where the data lives. `main` picks the backend. Tests use `MemoryStorage`.

### The error type

```rust
#[derive(Debug)]
pub enum TodoError {
    Io(std::io::Error),
    Parse(String),
    NotFound(u32),
}
```

Implement `Display`, `std::error::Error`, and `From<std::io::Error>` by hand
first. Then replace the boilerplate with `thiserror` and notice what it
generated for you.

### Concepts you must touch

- Derive macros: `Serialize`, `Deserialize`, `Parser`, `Subcommand`
- `impl From<E>` and how `?` uses it
- `Box<dyn Storage>` or generics `fn run<S: Storage>(store: &S)` — try both
- `filter`, `map`, `find`, `retain`, `collect` with closures
- `#[cfg(test)]` modules, `#[test]`, `assert_eq!`
- Reading docs on docs.rs for every crate you add
- Every Clippy lint it throws at you — read the explanation, don't just fix

### Definition of done

- Week 1 commands still work, now via `clap`
- `--help` output is useful
- At least 8 unit tests covering add/done/edit/remove/clear and error paths
- Zero Clippy warnings

---

## Week 3 — Stretch

**Pick at least two.** Pick the ones that scare you most.

**Read:** Rust Book chapter 15 (smart pointers), 17 (OOP patterns), 19
(advanced traits) as needed.

| Feature          | Example                              | Key concept                                 | Done |
| ---------------- | ------------------------------------ | ------------------------------------------- | :--: |
| Due dates        | `todo add "Tax" --due 2026-11-15`    | `chrono`, `FromStr`, `Ord` on dates         |  ⬜  |
| Tags             | `todo add "Fix" --tag work --tag p1` | `HashSet<String>`, `Vec` vs set trade-offs  |  ⬜  |
| Search           | `todo search milk`                   | Iterator chains, `to_lowercase`, `contains` |  ⬜  |
| Sort             | `todo list --sort due`               | `sort_by_key`, `Ord`/`PartialOrd`, `cmp`    |  ⬜  |
| Undo             | `todo undo`                          | `Clone`, snapshot history, `Vec` as stack   |  ⬜  |
| Subtasks         | `todo add "Sub" --parent 3`          | Trees via IDs or `Box<Task>`, recursion     |  ⬜  |
| Colored output   | —                                    | Extension traits (`"text".red()`)           |  ⬜  |
| Export           | `todo export --format csv`           | `Box<dyn Exporter>`, dynamic dispatch       |  ⬜  |
| Interactive REPL | `todo` with no args                  | `stdin().lines()`, loop + match, `BufRead`  |  ⬜  |
| Custom iterator  | `tasks.overdue()`                    | `impl Iterator for ...`, `type Item`        |  ⬜  |
| Config file      | `~/.config/todo/config.toml`         | `toml` crate, `dirs`, `Option` defaults     |  ⬜  |

### Definition of done

- Chosen features have tests
- Still Clippy clean
- Your `Storage` trait didn't need to change to add a backend (if you did
  SQLite) — if it did, write down why

---

## Week 4 — Read real code

**Read:** Rust Book chapter 16 (concurrency). Optionally chapter 20.

### Tasks

1. **Pick one real crate** and read it for at least three hours. Suggested
   by size:

   | Easier      | Medium  | Harder             |
   | ----------- | ------- | ------------------ |
   | `anyhow`    | `fd`    | `ripgrep` (`grep`) |
   | `thiserror` | `bat`   | `serde_json`       |
   | `clap_lex`  | `tokei` | `tokio` (one mod)  |

2. **Write `NOTES.md`** in your directory covering:
   - Patterns you recognized from your own project
   - Patterns you'd never seen — look them up and explain them
   - One thing you'd now change in your task manager because of it

3. **Final review.** Read both other participants' projects end to end.
   Leave at least five review comments or questions on each.

4. **Final sync.** Each person demos their tool and walks through their
   `NOTES.md`.

### Definition of done

- `NOTES.md` committed
- You can explain, without looking it up: ownership rules, the difference
  between `&`/`&mut`/owned, what `?` does, when you'd use `Box<dyn Trait>`
  vs generics, and why `Rc<RefCell<T>>` exists

---

## Rules recap

- Week 1 is `std` only. After that, use whatever crates you like.
- Refactor, don't rewrite, between weeks.
- No `unwrap()` in non-test code without a comment saying why it's safe.
- `cargo fmt` always. `cargo clippy -- -D warnings` from Week 2.
- Read each other's PRs. Don't copy each other's code.

---

## After the challenge — Swap files for SQLite (optional)

Once November is over, the real test of your `Storage` trait is whether you
can add a third backend without touching `commands.rs`.

**Read:** the [`rusqlite`](https://docs.rs/rusqlite) docs, and Rust Book
chapter 10.3 on lifetimes if it's still fuzzy.

### Tasks

| Task                                              | Key concept                                                                  | Done |
| ------------------------------------------------- | ---------------------------------------------------------------------------- | :--: |
| Add `rusqlite` with the `bundled` feature         | `-sys` crates, FFI, build scripts                                            |  ⬜  |
| Implement `SqliteStorage: Storage`                | Trait impl against a foreign type                                            |  ⬜  |
| `impl ToSql` / `FromSql` for `Status`, `Priority` | Traits on your enums for a crate's API                                       |  ⬜  |
| Map nullable columns to `Option<T>`               | `Option` in a new context                                                    |  ⬜  |
| Convert `rusqlite::Error` into `TodoError`        | `impl From`, growing an error enum                                           |  ⬜  |
| `todo migrate --from json --to sqlite`            | Two `Box<dyn Storage>` talking to each other                                 |  ⬜  |
| Try returning a `Statement` from a helper fn      | Feel `Statement<'conn>` borrow the connection — then understand why it fails |  ⬜  |

### Why this is the right finale

- You'll find out whether `load()`/`save()` as whole-list operations was the
  right abstraction, or whether a DB wants per-task `insert`/`update`. If
  you end up changing the trait, write down why — that's the lesson.
- `rusqlite` is a very idiomatic, well-documented crate. Reading its source
  is a good Week 4 follow-up.
- The lifetime on `Statement<'conn>` is one of the most instructive
  borrow-checker encounters available. Don't skip it.

### Definition of done

- `main` can select the backend with a flag or env var
- Existing tests still pass unchanged against `MemoryStorage`
- At least one integration test runs the same commands against both
  `JsonStorage` and `SqliteStorage` and asserts identical results
