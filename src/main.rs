mod task;
mod todo_list;

use crate::task::Status;
use crate::todo_list::TodoError;
use crate::todo_list::TodoList;
use std::io;
use std::io::Write;

const SAVE_PATH: &str = "todos.json";

enum Command {
    Add(String),
    Done(u32),
    Remove(u32),
    Progress(u32),
    List,
    Filter(Status),
    Quit,
    Unrecognized(String),
    InvalidArgument(String),
}

fn parse_command(input: &str) -> Command {
    let mut parts = input.splitn(2, ' ');
    let command = parts.next().unwrap_or("");
    let argument = parts.next().unwrap_or("").trim();

    match command {
        "add" => Command::Add(argument.to_string()),
        "done" => match argument.parse::<u32>() {
            Ok(id) => Command::Done(id),
            Err(_) => Command::InvalidArgument("done requires a numeric id".to_string()),
        },
        "remove" => match argument.parse::<u32>() {
            Ok(id) => Command::Remove(id),
            Err(_) => Command::InvalidArgument("remove requires a numeric id".to_string()),
        },
        "progress" => match argument.parse::<u32>() {
            Ok(id) => Command::Progress(id),
            Err(_) => Command::InvalidArgument("progress requires a numeric id".to_string()),
        },
        "filter" | "status" => match argument.to_lowercase().as_str() {
            "todo" => Command::Filter(Status::Todo),
            "done" => Command::Filter(Status::Done),
            "inprogress" | "progress" => Command::Filter(Status::InProgress),
            _ => Command::InvalidArgument("filter requires: todo | done | inprogress".to_string()),
        },
        "list" => Command::List,
        "quit" => Command::Quit,
        _ => Command::Unrecognized(command.to_string()),
    }
}

fn main() {
    let mut todo = TodoList::load_from_file(SAVE_PATH).unwrap_or_else(|_| TodoList::new());

    println!("Welcome to the Rust Todo List!");

    loop {
        println!(
            "\nCommands: add <desc> | done <id> | remove <id> | progress <id> | list | filter <todo|done|inprogress> | quit"
        );
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        match parse_command(input.trim()) {
            Command::Add(desc) => {
                if desc.is_empty() {
                    println!("Error: description cannot be empty.");
                } else {
                    todo.add_task(desc);
                    println!("Task added.");
                }
            }
            Command::Done(id) => match todo.complete_task(id) {
                Ok(()) => println!("Task {} marked as done.", id),
                Err(TodoError::TaskNotFound(id)) => println!("Error: task {} not found.", id),
            },
            Command::Remove(id) => match todo.remove_task(id) {
                Ok(()) => println!("Task {} removed.", id),
                Err(TodoError::TaskNotFound(id)) => println!("Error: task {} not found.", id),
            },
            Command::Progress(id) => match todo.set_in_progress(id) {
                Ok(()) => println!("Task {} marked as in progress.", id),
                Err(TodoError::TaskNotFound(id)) => println!("Error: task {} not found.", id),
            },
            Command::List => {
                let lines = todo.list_tasks();
                TodoList::print_lines(lines);
            }
            Command::Filter(status) => {
                let lines = todo.list_by_status(status);
                if lines.is_empty() {
                    println!("No tasks found with status '{}'.", status);
                } else {
                    TodoList::print_lines(lines);
                }
            }
            Command::Quit => {
                if let Err(e) = todo.save_to_file(SAVE_PATH) {
                    println!("Warning: failed to save tasks: {}", e);
                }
                println!("Goodbye!");
                break;
            }
            Command::Unrecognized(cmd) => println!("Unknown command: '{}'.", cmd),
            Command::InvalidArgument(msg) => println!("Invalid argument: {}.", msg),
        }
    }
}
