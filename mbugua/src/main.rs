mod generate_uuid;

use generate_uuid::generate_uuid;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};

// Control accepted commands
enum Command {
    List,
    Add,
    Update,
    Check,
    Remove,
}

#[derive(Debug)]
struct ToDo {
    id: String,
    title: String,
    status: bool,
}



fn main() {

    // Manage our array of todos
    let mut todo_arr: Vec<ToDo> = Vec::new();

    // File management:
    let file = File::open("todos.txt").expect("Could not open file");
    let reader = BufReader::new(&file);

    // Read todos from file
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        if line.trim().is_empty() {
            continue;
        }

        let mut fields = line.split('#');

        let id = fields
            .next().unwrap()
            .to_string();

        let task = fields.next().expect("Missing task").to_string();

        let status = fields
            .next()
            .expect("Missing status")
            .parse()
            .expect("Invalid Status");

        // Keep the parsed task in memory for the requested command.
        todo_arr.push(ToDo {
            id,
            title: task,
            status,
        });
    }

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

    // pass the todos to run; it reports whether anything changed
    let changed = run(&cmd, &mut todo_arr);

    // Only save when a command actually modified the todos.
    if changed {
        let file = File::create("todos.txt").expect("Could not create file");
        for task in todo_arr {
            writeln!(&file, "{}#{}#{}", task.id, task.title, task.status)
                .expect("Could not write to file");
        }
    }
}

// Execute the entered command
// Returns true if the todos were modified and need saving.
fn run(cmd: &Command, todo_arr: &mut Vec<ToDo>) -> bool {
    match cmd {
        Command::List => {
            list_todos(todo_arr);
            false
        }
        Command::Add => add_todo(todo_arr),
        Command::Update => update_todo(todo_arr),
        Command::Check => mark_complete(todo_arr),
        Command::Remove => remove(todo_arr),
    }
}

// Actions
fn list_todos(todo_arr: &Vec<ToDo>) {
    for todo in todo_arr {
        println!(
            "{} [{}] {}",
            todo.id,
            if todo.status { "+" } else { " " },
            todo.title
        );
    }
}

fn add_todo(todo_arr: &mut Vec<ToDo>) -> bool {
    let mut input = String::new();

    println!("Please type in your todo:::");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed_todo = input.trim();
    let new_todo = ToDo {
        id: generate_uuid(),
        title: trimmed_todo.to_string(),
        status: false,
    };

    todo_arr.push(new_todo);

    println!("Todo added successfully!");
    true
}

fn remove(todo_arr: &mut Vec<ToDo>) -> bool {
    println!("Here is the list of todos:");
    list_todos(todo_arr);
    println!("Please type in the id of the todo you want to remove:::");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let index = input.trim().to_string();

    if !todo_arr.iter().any(|x| x.id == index) {
        println!("Unable to find an item with that id");
        false
    } else {
        todo_arr.retain(|item| item.id != index);
        println!("Todo removed successfully!");
        true
    }
}

fn update_todo(todo_arr: &mut Vec<ToDo>) -> bool {
    println!("Here is the list of todos:");
    list_todos(todo_arr);
    println!("Please type in the id of the todo you want to update:::");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let index = input.trim().to_string();

    if todo_arr.iter().find(|x| x.id == index).is_none() {
        println!("Unable to find an item with that id");
        false
    } else {
        println!("{}", index);
        println!("Please type in the updated text for your todo");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let updated_todo = input.trim().to_string();
        println!("{}", updated_todo);

        // println!("Todo removed successfully!");
        // println!("{:?}", todo_arr);
        // TODO: apply the update; nothing is modified yet.
        false
    }
}

fn mark_complete(todo_arr: &mut Vec<ToDo>) -> bool {
    println!("Here is the list of todos:");
    list_todos(todo_arr);
    println!("Please provide the id of the task you would like to mark as complete:::");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let id = input.trim();

    match todo_arr.iter_mut().find(|item| item.id == id) {
        Some(item) if item.status => {
            println!("Todo is already complete.");
            false
        }
        Some(item) => {
            item.status = true;
            println!("Todo marked as complete!");
            true
        }
        None => {
            println!("Unable to find an item with that id");
            false
        }
    }
}
