//! Interactive command-line entry point for the todo app.
//!
//! Loads any previously saved list from [`SAVE_PATH`] on startup, runs a
//! REPL loop that parses and executes user commands, and saves the list
//! back to disk when the user quits.

mod task;
mod todo_list;

use crate::task::Status;
use crate::todo_list::TodoError;
use crate::todo_list::TodoList;
use std::io;
use std::io::Write;

/// Path to the JSON file used to persist the todo list between runs.
const SAVE_PATH: &str = "todos.json";

/// A parsed user command, produced by [`parse_command`] from a line of
/// raw input.
enum Command {
    /// Add a new task with the given description.
    Add(String),
    /// Mark the task with the given id as done.
    Done(u32),
    /// Remove the task with the given id.
    Remove(u32),
    /// Mark the task with the given id as in progress.
    Progress(u32),
    /// List all tasks.
    List,
    /// List only tasks with the given status.
    Filter(Status),
    /// Exit the program (saving first).
    Quit,
    /// The command name itself was not recognized.
    Unrecognized(String),
    /// The command was recognized but its argument was missing or invalid.
    InvalidArgument(String),
}

/// Parses a raw line of user input into a [`Command`].
///
/// The first whitespace-separated token is treated as the command name;
/// everything after it (trimmed) is the argument. Unknown command names
/// produce [`Command::Unrecognized`]; known commands with a bad or missing
/// argument produce [`Command::InvalidArgument`].
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

/// Runs the interactive todo list REPL.
///
/// Loads [`SAVE_PATH`] on startup (or starts with an empty list if it
/// doesn't exist yet), then loops reading a line of input, parsing it via
/// [`parse_command`], and executing the corresponding [`TodoList`]
/// operation until the user issues `quit`, at which point the list is
/// saved back to [`SAVE_PATH`].
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
