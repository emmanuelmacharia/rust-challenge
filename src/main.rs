use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

// Control accepted commands
enum Command {
    List,
    Add,
    Remove,
}

#[derive(Debug)]
struct ToDo {
    id: u32,
    title: String,
    status: bool,
}

fn main() {
    // Manage our array of todos
    let mut todo_arr: Vec<ToDo> = Vec::new();

    // File management:
    let file = File::open("todos.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    // Read todos from file
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        if line.trim().is_empty() {
            continue;
        }

        let mut fields = line.split('#');

        let id = fields
            .next()
            .expect("Could not find id")
            .parse::<u32>()
            .expect("Invalid ID");

        let task = fields.next().expect("Missing task").to_string();

        let status = fields
            .next()
            .expect("Missing status")
            .parse()
            .expect("Invalid Status");

        // Keep the parsed todo in memory for the requested command.
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
    let cmd = match args[1].as_str() {
        "list" => Command::List,
        "add" => Command::Add,
        "remove" => Command::Remove,
        other => {
            eprintln!("Unknown command: {}", other);
            std::process::exit(1);
        }
    };

    // pass the todos to run
    run(cmd, &mut todo_arr);
}

// Execute the entered command
fn run(cmd: Command, todo_arr: &mut Vec<ToDo>) {
    match cmd {
        Command::List => list_todos(todo_arr),
        Command::Add => add_todo(todo_arr),
        Command::Remove => remove(todo_arr),
    }
}

// Actions
fn list_todos(todo_arr: &Vec<ToDo>) {
    for todo in todo_arr {
        println!(
            "{}. [{}] {}",
            todo.id,
            if todo.status { "+" } else { " " },
            todo.title
        );
    }
}

fn add_todo(todo_arr: &mut Vec<ToDo>) {
    let mut input = String::new();

    println!("Please type in your todo:::");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed_todo = input.trim();
    let new_todo = ToDo {
        id: todo_arr.len() as u32,
        title: trimmed_todo.to_string(),
        status: false,
    };

    todo_arr.push(new_todo);

    println!("Todo added successfully!");
    println!("{:?}", todo_arr);
}

fn remove(todo_arr: &mut Vec<ToDo>) {
    println!("Here is the list of todos:");
    println!("{:?}", todo_arr);
    println!("Please type in the id of the todo you want to remove:::");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let index = input.trim().parse::<usize>().unwrap();
    todo_arr.remove(index - 1);

    println!("Todo removed successfully!");
    println!("{:?}", todo_arr);
}
