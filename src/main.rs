enum Command {
    List,
}

#[derive(Debug)]
struct ToDo {
    id: u32,
    title: String,
    status: bool,
}

fn main() {
    // Array of different commands from terminal
    let args: Vec<String> = std::env::args().collect();

    // if an argument is not provided
    if args.len() < 2 {
        eprintln!("Usage: todo_list <command> [args]");
        std::process::exit(1);
    }

    // argument provided
    let cmd = match args[1].as_str() {
        "list" => Command::List,
        other => {
            eprintln!("Unknown command: {}", other);
            std::process::exit(1);
        }
    };

    run(cmd);
}

// Execute the entered command
fn run(cmd: Command) {
    match cmd {
        Command::List => {
            println!("{:?}", list_todos())
        },
    };
}



// Actions
fn list_todos() -> Vec<ToDo> {
    let mut todo_arr: Vec<ToDo> =  Vec::new();
    let first_todo = ToDo {
        id: 1,
        title: String::from("Go to Work"),
        status: false,
    };
    todo_arr.push(first_todo);
    todo_arr
}