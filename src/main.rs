mod task;
mod todo_list;

use crate::task::Status;
use crate::todo_list::TodoList;
use std::io;
use std::io::Write;

enum Command {
    Add(String),
    Done(u32),
    Remove(u32),
    Progress(u32),
    List,
    Filter(Status),
    Quit,
    Unknown,
}

fn parse_command(input: &str) -> Command {
    let mut parts = input.splitn(2, ' ');
    let command = parts.next().unwrap_or("");
    let argument = parts.next().unwrap_or("").trim();

    match command {
        "add" => Command::Add(argument.to_string()),

        "done" => match argument.parse::<u32>() {
            Ok(id) => Command::Done(id),
            Err(_) => Command::Unknown,
        },

        "remove" => match argument.parse::<u32>() {
            Ok(id) => Command::Remove(id),
            Err(_) => Command::Unknown,
        },

        "progress" => match argument.parse::<u32>() {
            Ok(id) => Command::Progress(id),
            Err(_) => Command::Unknown,
        },

        "filter" | "status" => match argument.to_lowercase().as_str() {
            "todo" => Command::Filter(Status::Todo),
            "done" => Command::Filter(Status::Done),
            "inprogress" | "progress" => Command::Filter(Status::InProgress),
            _ => Command::Unknown,
        },

        "list" => Command::List,
        "quit" => Command::Quit,
        _ => Command::Unknown,
    }
}

fn main() {
    let mut todo = TodoList::new();

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
            Command::Done(id) => todo.complete_task(id),
            Command::Remove(id) => todo.remove_task(id),
            Command::Progress(id) => todo.set_in_progress(id),
            Command::List => {
                let lines = todo.list_tasks();
                TodoList::print_lines(lines);
            }

            Command::Filter(status) => {
                let lines = todo.list_by_status(status);
                if lines.is_empty() {
                    // Note: Requires #[derive(Debug)] on Status
                    println!("No tasks found with status '{:?}'.", status);
                } else {
                    TodoList::print_lines(lines);
                }
            }
            Command::Quit => {
                println!("Goodbye!");
                break;
            }
            Command::Unknown => println!(
                "Unknown command. Use 'filter todo', 'filter done', or 'filter inprogress'."
            ),
        }
    }
}
