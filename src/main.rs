//! Interactive command-line entry point for the todo app.
//!
//! Loads any previously saved list from [`SAVE_PATH`] on startup, runs a
//! REPL loop that parses and executes user commands, and saves the list
//! back to disk when the user quits.

mod board;
mod task;
mod todo_list;

use crate::board::Board;
use crate::task::Status;
use crate::todo_list::TodoError;
use crate::todo_list::TodoList;
use std::io;
use std::io::Write;

/// Path to the JSON file used to persist the todo list between runs.
const SAVE_PATH: &str = "todos.json";
/// Path to the JSON file used to persist the multi-list board between runs.
const BOARD_SAVE_PATH: &str = "board.json";

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

    // --- Board (Chapter 15) commands ---
    /// `badd <list> <description>` — add a new task directly into a
    /// named list (creating the list if needed).
    BoardAdd(String, String),
    /// `bdone <id>` — mark a board task done, visible from every list
    /// that references it.
    BoardDone(u32),
    /// `blist <list>` — show tasks in a named list.
    BoardList(String),
    /// `blists` — show the names of every list that currently exists.
    BoardLists,
    /// `bassign <id> <list>` — share an existing task into another list.
    BoardAssign(u32, String),
    /// `bwhere <id>` — show which lists reference a given task id.
    BoardWhere(u32),
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

        "badd" => {
            // argument is "<list> <description>" — split once more on
            // the first remaining space.
            let mut sub = argument.splitn(2, ' ');
            let list_name = sub.next().unwrap_or("").trim();
            let desc = sub.next().unwrap_or("").trim();
            if list_name.is_empty() || desc.is_empty() {
                Command::InvalidArgument("badd requires: <list> <description>".to_string())
            } else {
                Command::BoardAdd(list_name.to_string(), desc.to_string())
            }
        }

        "bdone" => match argument.parse::<u32>() {
            Ok(id) => Command::BoardDone(id),
            Err(_) => Command::InvalidArgument("bdone requires a numeric id".to_string()),
        },

        "blist" => {
            if argument.is_empty() {
                Command::InvalidArgument("blist requires: <list>".to_string())
            } else {
                Command::BoardList(argument.to_string())
            }
        }

        "blists" => Command::BoardLists,

        "bassign" => {
            let mut sub = argument.splitn(2, ' ');
            let id_str = sub.next().unwrap_or("").trim();
            let list_name = sub.next().unwrap_or("").trim();
            match (id_str.parse::<u32>(), list_name.is_empty()) {
                (Ok(id), false) => Command::BoardAssign(id, list_name.to_string()),
                _ => Command::InvalidArgument("bassign requires: <id> <list>".to_string()),
            }
        }

        "bwhere" => match argument.parse::<u32>() {
            Ok(id) => Command::BoardWhere(id),
            Err(_) => Command::InvalidArgument("bwhere requires a numeric id".to_string()),
        },

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
    let mut board = Board::load_from_file(BOARD_SAVE_PATH).unwrap_or_else(|_| Board::new());

    println!("Welcome to the Rust Todo List!");

    loop {
        println!(
            "\nCommands: add <desc> | done <id> | remove <id> | progress <id> | list | filter <todo|done|inprogress> | quit"
        );
        println!(
            "Board commands: badd <list> <desc> | bdone <id> | blists | blist <list> | bassign <id> <list> | bwhere <id>"
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
                if let Err(e) = board.save_to_file(BOARD_SAVE_PATH) {
                    println!("Warning: failed to save board: {}", e);
                }
                println!("Goodbye!");
                break;
            }
            Command::Unrecognized(cmd) => println!("Unknown command: '{}'.", cmd),
            Command::InvalidArgument(msg) => println!("Invalid argument: {}.", msg),

            Command::BoardAdd(list_name, desc) => {
                let id = board.add_task(&list_name, desc);
                println!("Task {} added to list '{}'.", id, list_name);
            }
            Command::BoardDone(id) => match board.complete_task(id) {
                Ok(()) => println!("Task {} marked as done (all lists).", id),
                Err(TodoError::TaskNotFound(id)) => println!("Error: task {} not found.", id),
            },
            Command::BoardList(list_name) => {
                let lines = board.list_tasks(&list_name);
                if lines.is_empty() {
                    println!("No tasks in list '{}'.", list_name);
                } else {
                    TodoList::print_lines(lines);
                }
            }
            Command::BoardLists => {
                let names = board.list_names();
                if names.is_empty() {
                    println!("No lists yet.");
                } else {
                    println!("Lists ({}): {}", names.len(), names.join(", "));
                }
            }
            Command::BoardAssign(id, list_name) => {
                match board.assign_task_to_list(id, &list_name) {
                    Ok(()) => println!("Task {} shared into list '{}'.", id, list_name),
                    Err(TodoError::TaskNotFound(id)) => println!("Error: task {} not found.", id),
                }
            }
            Command::BoardWhere(id) => {
                let lists = board.lists_containing(id);
                if lists.is_empty() {
                    println!("Task {} is not in any list.", id);
                } else {
                    println!("Task {} is in: {}", id, lists.join(", "));
                }
            }
        }
    }
}
