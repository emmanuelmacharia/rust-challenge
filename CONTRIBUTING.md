# Contributing

This is a shared learning repo, not a shared codebase. Three people, three
independent projects, one place to read and review each other's work.

## Getting set up

```bash
# Install Rust (if you haven't)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable

# Clone and create your project
git clone git@github.com:<org>/rustvember.git
cd rustvember
cargo new <your-name>
```

Commit the generated `Cargo.toml`, `src/main.rs`, and add a `README.md` in
your directory describing your binary name and how to run it.

## Directory ownership

| Path              | Owner    | How to change it           |
| ----------------- | -------- | -------------------------- |
| `<your-name>/`    | You      | PR, self-merge after 1 day |
| `README.md`       | Everyone | PR, 1 approval             |
| `CHALLENGE.md`    | Everyone | PR, 2 approvals            |
| `CONTRIBUTING.md` | Everyone | PR, 2 approvals            |
| `.github/`        | Everyone | PR, 1 approval             |

Never commit into someone else's directory. If you spot a bug in their code,
leave a review comment or open an issue — don't fix it for them.

## Workflow

1. Create a branch: `<name>/<feature>` (e.g. `alice/json-storage`)
2. Make small, focused commits
3. Before pushing:
   ```bash
   cargo fmt
   cargo clippy -- -D warnings   # from Week 2 onward
   cargo test
   ```
4. Open a PR against `main`, even if it's your own code
5. Tag the other two as reviewers
6. Merge after at least one review **or** 24 hours, whichever comes first

## Pull request template

Use this as the PR description:

```markdown
## What

One or two sentences on the feature.

## Rust concepts I hit

- e.g. "First time using `impl From<io::Error>` for my error enum"

## Where I struggled

- e.g. "Couldn't return `&Task` from a method that took `&mut self` — why?"

## Questions for reviewers

- Anything you'd like the others to weigh in on
```

## Reviewing

Reviews are the main learning tool in this repo. When reviewing:

- **Ask, don't dictate.** "Why `clone()` here instead of a borrow?" beats
  "Remove this clone."
- **Point at idioms.** If you know a more idiomatic form, link to the docs
  or Clippy lint, not your own code.
- **Compare approaches.** "I did this with `retain`, you used
  `filter().collect()` — want to compare trade-offs?"
- **Approve generously.** A PR that compiles, has tests, and taught the
  author something is a good PR.

Don't block on style the author hasn't learned yet. Block only on:

- Code that doesn't compile or tests that fail
- `unwrap()` / `expect()` in non-test code without a comment justifying it
- Commits to someone else's directory

## Commit messages

Imperative mood, under 72 characters, describe _what_ not _how_:

```text
Add Storage trait with JSON and in-memory impls
Replace manual arg parsing with clap derive
Fix off-by-one in next_id when list is empty
```

## Asking for help

Order of operations when stuck:

1. Read the **full** compiler error, including the `help:` lines
2. Run `cargo clippy` and read what it says
3. Search the error code: `rustc --explain E0502`
4. Check the Rust Book chapter for the concept
5. Open a GitHub Discussion or drop it in the group chat with the error
   pasted verbatim

Twenty minutes stuck is normal. Two hours stuck is when you ask.

## Issues and Discussions

- **Issues** are for bugs in your own project or problems with the shared
  files. Prefix with your name: `[alice] Panic on empty tasks file`
- **Discussions** are for "how did you approach X?" conversations and
  weekly sync notes

## Weekly sync

Mondays. Each person brings:

- A 3-minute demo
- One borrow-checker fight (and how you won or gave up)
- One thing you read in someone else's PR that you want to steal — then
  re-implement it yourself, don't copy it
