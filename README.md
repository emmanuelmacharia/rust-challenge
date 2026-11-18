# todo_list

A small command-line todo list written in Rust, using only the standard
library.

- **Binary name:** `todo_list`
- **Storage:** plain-text file `todos.txt` in the directory you run it from
  (it must exist, even if empty)

## Running

```bash
cargo run -- <command>
```

| Command  | What it does                                              |
| -------- | --------------------------------------------------------- |
| `list`   | Print all todos                                           |
| `add`    | Prompt for a title and add a todo                         |
| `update` | Show todos, prompt for an id, then for the new title      |
| `check`  | Show todos, prompt for an id, and mark that todo complete |
| `remove` | Show todos, prompt for an id, and delete that todo        |

Commands that need input prompt for it on stdin, so they can also be piped:

```bash
printf 'Buy milk\n' | cargo run -- add
cargo run -- list
```

Example output of `list` (`+` means complete):

```text
bb473fb1-7c06-42b6-b28d-7a4696060d16 [+] charge my buds
e0d08d7b-50b7-4bb8-821b-dc7aac78dabd [ ] Buy milk
```

## File format

One todo per line: `id#title#status`, where `id` is a UUID v4 and `status`
is `true` or `false`. Titles must not contain `#`. The file is only rewritten
when a command changes something.

## Development

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```
