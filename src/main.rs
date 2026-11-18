mod generate_uuid;

use generate_uuid::generate_uuid;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};

// Control accepted commands
enum Command {
    List,
    Add,
    Update,
    Check,
    Remove,
}

struct ToDo {
    id: String,
    title: String,
    status: bool,
}

const TODO_FILE: &str = "todos.txt";

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn try_main() -> io::Result<()> {
    // Array of different commands from terminal
    let args: Vec<String> = std::env::args().collect();

    // if an argument is not provided print error on console
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <command>");
        std::process::exit(1);
    }

    // argument provided:: get command and send to run function
    let cmd: Command = match args[1].as_str() {
        "list" => Command::List,
        "add" => Command::Add,
        "update" => Command::Update,
        "check" => Command::Check,
        "remove" => Command::Remove,
        other => {
            eprintln!("Unknown command: {}", other);
            std::process::exit(1);
        }
    };

    // Manage our array of todos
    let mut todo_arr = load_todos()?;

    // pass the todos to run; it reports whether anything changed
    let changed = run(&cmd, &mut todo_arr)?;

    // Only save when a command actually modified the todos.
    if changed {
        save_todos(&todo_arr)?;
    }

    Ok(())
}

// Parse one `id#title#status` line. The title may itself contain '#',
// so split the id off the front and the status off the back.
fn parse_line(line: &str) -> Result<ToDo, String> {
    let (id, rest) = line.split_once('#').ok_or("missing title")?;
    let (title, status) = rest.rsplit_once('#').ok_or("missing status")?;
    let status = status
        .parse()
        .map_err(|_| format!("invalid status '{}'", status))?;

    Ok(ToDo {
        id: id.to_string(),
        title: title.to_string(),
        status,
    })
}

fn format_line(todo: &ToDo) -> String {
    format!("{}#{}#{}", todo.id, todo.title, todo.status)
}

// Read todos from file. A missing file just means there are no todos yet.
fn load_todos() -> io::Result<Vec<ToDo>> {
    let file = match File::open(TODO_FILE) {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };

    let mut todo_arr = Vec::new();
    for (index, line) in BufReader::new(file).lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let todo = parse_line(&line).map_err(|msg| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{} line {}: {}", TODO_FILE, index + 1, msg),
            )
        })?;
        todo_arr.push(todo);
    }

    Ok(todo_arr)
}

// Write to a temp file then rename, so a crash mid-write can't lose todos.
fn save_todos(todo_arr: &[ToDo]) -> io::Result<()> {
    let tmp_path = format!("{}.tmp", TODO_FILE);
    let mut file = File::create(&tmp_path)?;
    for todo in todo_arr {
        writeln!(file, "{}", format_line(todo))?;
    }
    file.sync_all()?;
    fs::rename(&tmp_path, TODO_FILE)
}

// Print a message and read one trimmed line from stdin.
fn prompt(message: &str) -> io::Result<String> {
    println!("{}", message);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

// Execute the entered command
// Returns true if the todos were modified and need saving.
fn run(cmd: &Command, todo_arr: &mut Vec<ToDo>) -> io::Result<bool> {
    match cmd {
        Command::List => {
            list_todos(todo_arr);
            Ok(false)
        }
        Command::Add => add_todo(todo_arr),
        Command::Update => update_todo(todo_arr),
        Command::Check => mark_complete(todo_arr),
        Command::Remove => remove(todo_arr),
    }
}

// Actions
fn list_todos(todo_arr: &[ToDo]) {
    if todo_arr.is_empty() {
        println!("No todos yet.");
    }
    for todo in todo_arr {
        println!(
            "{} [{}] {}",
            todo.id,
            if todo.status { "+" } else { " " },
            todo.title
        );
    }
}

fn add_todo(todo_arr: &mut Vec<ToDo>) -> io::Result<bool> {
    let title = prompt("Please type in your todo:::")?;

    if title.is_empty() {
        println!("A todo cannot be empty. Nothing added.");
        return Ok(false);
    }

    todo_arr.push(ToDo {
        id: generate_uuid(),
        title,
        status: false,
    });

    println!("Todo added successfully!");
    Ok(true)
}

fn remove(todo_arr: &mut Vec<ToDo>) -> io::Result<bool> {
    println!("Here is the list of todos:");
    list_todos(todo_arr);
    let id = prompt("Please type in the id of the todo you want to remove:::")?;

    if !todo_arr.iter().any(|x| x.id == id) {
        println!("Unable to find an item with that id");
        return Ok(false);
    }

    todo_arr.retain(|item| item.id != id);
    println!("Todo removed successfully!");
    Ok(true)
}

fn update_todo(todo_arr: &mut [ToDo]) -> io::Result<bool> {
    println!("Here is the list of todos:");
    list_todos(todo_arr);
    let id = prompt("Please type in the id of the todo you want to update:::")?;

    let Some(item) = todo_arr.iter_mut().find(|item| item.id == id) else {
        println!("Unable to find an item with that id");
        return Ok(false);
    };

    println!("Current text: {}", item.title);
    let updated = prompt("Please type in the updated text for your todo:::")?;

    if updated.is_empty() {
        println!("Updated text cannot be empty. Nothing changed.");
        return Ok(false);
    }

    if updated == item.title {
        println!("Text is unchanged.");
        return Ok(false);
    }

    item.title = updated;
    println!("Todo updated successfully!");
    Ok(true)
}

fn mark_complete(todo_arr: &mut [ToDo]) -> io::Result<bool> {
    println!("Here is the list of todos:");
    list_todos(todo_arr);
    let id = prompt("Please provide the id of the task you would like to mark as complete:::")?;

    match todo_arr.iter_mut().find(|item| item.id == id) {
        Some(item) if item.status => {
            println!("Todo is already complete.");
            Ok(false)
        }
        Some(item) => {
            item.status = true;
            println!("Todo marked as complete!");
            Ok(true)
        }
        None => {
            println!("Unable to find an item with that id");
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_valid_line() {
        let todo = parse_line("abc#buy milk#true").unwrap();
        assert_eq!(todo.id, "abc");
        assert_eq!(todo.title, "buy milk");
        assert!(todo.status);
    }

    #[test]
    fn title_may_contain_hash() {
        let todo = parse_line("abc#fix bug #42#false").unwrap();
        assert_eq!(todo.title, "fix bug #42");
        assert!(!todo.status);
    }

    #[test]
    fn line_round_trips() {
        let todo = parse_line("abc#a #tag#false").unwrap();
        assert_eq!(format_line(&todo), "abc#a #tag#false");
    }

    #[test]
    fn rejects_malformed_lines() {
        assert!(parse_line("just-an-id").is_err());
        assert!(parse_line("abc#no status").is_err());
        assert!(parse_line("abc#title#maybe").is_err());
    }
}
