// Control accepted commands
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
    // Manage our array of todos
    let mut todo_arr: Vec<ToDo> =  Vec::new();
    let first_todo = ToDo {
        id: 1,
        title: String::from("Go to Work"),
        status: false,
    };
    todo_arr.push(first_todo);


    // Array of different commands from terminal
    let args: Vec<String> = std::env::args().collect();

    // if an argument is not provided print error on console
    if args.len() < 2 {
        eprintln!("Usage: todo_list <command> [args]");
        std::process::exit(1);
    }

    // argument provided:: get command and send to run function
    let cmd = match args[1].as_str() {
        "list" => Command::List,
        other => {
            eprintln!("Unknown command: {}", other);
            std::process::exit(1);
        }
    };

    // pass the todos to run
    run(cmd, &todo_arr);
}

// Execute the entered command
fn run(cmd: Command, todo_arr: &Vec<ToDo>) {
    match cmd {
        Command::List => list_todos(todo_arr),
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