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
use crate::todo_list::TodoList;
use colored::Colorize;
use std::io;
use std::io::Write;
use std::path::PathBuf;

/// Returns the directory used to store this app's data files
/// (`~/.todo_cli` on Linux/macOS), creating it if it doesn't exist yet.
///
/// Falls back to the current directory if `HOME` isn't set, so the
/// program still runs (just without the "works from anywhere" benefit).
fn data_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let dir = PathBuf::from(home).join(".todo_cli");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

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
    /// Show the help menu.
    Help,
    /// The command name itself was not recognized.
    Unrecognized(String),
    /// The command was recognized but its argument was missing or invalid.
    InvalidArgument(String),

    // --- Board (Chapter 15) commands ---
    /// `bcreate <list>` — create an empty named list.
    BoardCreate(String),
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
    /// `bunassign <id> <list>` — remove a task from one list, keeping it
    /// in others (and deleting it entirely if that was the last one).
    BoardUnassign(u32, String),
    /// `bremove <id>` — remove a task entirely, from every list.
    BoardRemove(u32),
    /// `bdelete <list>` — delete an entire list (unassigning its tasks first).
    BoardDeleteList(String),
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
        "help" | "?" => Command::Help,

        "bcreate" => {
            if argument.is_empty() {
                Command::InvalidArgument("bcreate requires: <list>".to_string())
            } else {
                Command::BoardCreate(argument.to_string())
            }
        }

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

        "bunassign" => {
            let mut sub = argument.splitn(2, ' ');
            let id_str = sub.next().unwrap_or("").trim();
            let list_name = sub.next().unwrap_or("").trim();
            match (id_str.parse::<u32>(), list_name.is_empty()) {
                (Ok(id), false) => Command::BoardUnassign(id, list_name.to_string()),
                _ => Command::InvalidArgument("bunassign requires: <id> <list>".to_string()),
            }
        }

        "bremove" => match argument.parse::<u32>() {
            Ok(id) => Command::BoardRemove(id),
            Err(_) => Command::InvalidArgument("bremove requires a numeric id".to_string()),
        },

        "bdelete" => {
            if argument.is_empty() {
                Command::InvalidArgument("bdelete requires: <list>".to_string())
            } else {
                Command::BoardDeleteList(argument.to_string())
            }
        }

        "bwhere" => match argument.parse::<u32>() {
            Ok(id) => Command::BoardWhere(id),
            Err(_) => Command::InvalidArgument("bwhere requires a numeric id".to_string()),
        },

        _ => Command::Unrecognized(command.to_string()),
    }
}

/// Prints the full help menu, grouped by section.
fn print_help() {
    println!("\n{}", "── Basic list ──".bold());
    println!("  {:<28} add a new task", "add <desc>".cyan());
    println!("  {:<28} mark a task done", "done <id>".cyan());
    println!("  {:<28} remove a task", "remove <id>".cyan());
    println!("  {:<28} mark a task in progress", "progress <id>".cyan());
    println!("  {:<28} show all tasks", "list".cyan());
    println!(
        "  {:<28} show tasks with a given status",
        "filter <todo|done|inprogress>".cyan()
    );

    println!("\n{}", "── Multi-list board ──".bold());
    println!("  {:<28} create an empty list", "bcreate <list>".magenta());
    println!(
        "  {:<28} add a task directly into a list",
        "badd <list> <desc>".magenta()
    );
    println!(
        "  {:<28} mark a board task done (all lists see it)",
        "bdone <id>".magenta()
    );
    println!("  {:<28} show every list's name", "blists".magenta());
    println!("  {:<28} show tasks in a list", "blist <list>".magenta());
    println!(
        "  {:<28} share an existing task into another list",
        "bassign <id> <list>".magenta()
    );
    println!(
        "  {:<28} remove a task from one list only",
        "bunassign <id> <list>".magenta()
    );
    println!(
        "  {:<28} remove a task entirely (all lists)",
        "bremove <id>".magenta()
    );
    println!(
        "  {:<28} delete a whole list (unassigns its tasks first)",
        "bdelete <list>".magenta()
    );
    println!(
        "  {:<28} show which lists contain a task",
        "bwhere <id>".magenta()
    );

    println!("\n{}", "── Other ──".bold());
    println!("  {:<28} show this help", "help / ?".yellow());
    println!("  {:<28} save and exit", "quit".yellow());
}

fn main() {
    let todos_path = data_dir().join("todos.json");
    let board_path = data_dir().join("board.json");
    let todos_path = todos_path.to_string_lossy().to_string();
    let board_path = board_path.to_string_lossy().to_string();

    let mut todo = TodoList::load_from_file(&todos_path).unwrap_or_else(|_| TodoList::new());
    let mut board = Board::load_from_file(&board_path).unwrap_or_else(|_| Board::new());

    println!("{}", "Welcome to the Rust Todo List!".bold().green());
    println!("Type {} for the list of commands.", "help".yellow());

    loop {
        print!("\n{} ", ">".bold().blue());
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        match parse_command(input.trim()) {
            Command::Add(desc) => {
                if desc.is_empty() {
                    println!("{}", "Error: description cannot be empty.".red());
                } else {
                    todo.add_task(desc);
                    println!("{}", "Task added.".green());
                }
            }
            Command::Done(id) => match todo.complete_task(id) {
                Ok(()) => println!("{}", format!("Task {} marked as done.", id).green()),
                Err(e) => println!("{}", format!("Error: {}.", e).red()),
            },
            Command::Remove(id) => match todo.remove_task(id) {
                Ok(()) => println!("{}", format!("Task {} removed.", id).green()),
                Err(e) => println!("{}", format!("Error: {}.", e).red()),
            },
            Command::Progress(id) => match todo.set_in_progress(id) {
                Ok(()) => println!("{}", format!("Task {} marked as in progress.", id).green()),
                Err(e) => println!("{}", format!("Error: {}.", e).red()),
            },
            Command::List => {
                let lines = todo.list_tasks();
                TodoList::print_lines(lines);
            }
            Command::Filter(status) => {
                let lines = todo.list_by_status(status);
                if lines.is_empty() {
                    println!(
                        "{}",
                        format!("No tasks found with status '{}'.", status).yellow()
                    );
                } else {
                    TodoList::print_lines(lines);
                }
            }
            Command::Help => print_help(),
            Command::Quit => {
                if let Err(e) = todo.save_to_file(&todos_path) {
                    println!("{}", format!("Warning: failed to save tasks: {}", e).red());
                }
                if let Err(e) = board.save_to_file(&board_path) {
                    println!("{}", format!("Warning: failed to save board: {}", e).red());
                }
                println!("{}", "Goodbye!".bold().green());
                break;
            }
            Command::Unrecognized(cmd) => println!(
                "{}",
                format!("Unknown command: '{}'. Type 'help' for the list.", cmd).red()
            ),
            Command::InvalidArgument(msg) => {
                println!("{}", format!("Invalid argument: {}.", msg).red())
            }

            Command::BoardCreate(list_name) => {
                board.create_list(&list_name);
                println!("{}", format!("List '{}' created.", list_name).green());
            }
            Command::BoardAdd(list_name, desc) => {
                let id = board.add_task(&list_name, desc);
                println!(
                    "{}",
                    format!("Task {} added to list '{}'.", id, list_name).green()
                );
            }
            Command::BoardDone(id) => match board.complete_task(id) {
                Ok(()) => println!(
                    "{}",
                    format!("Task {} marked as done (all lists).", id).green()
                ),
                Err(e) => println!("{}", format!("Error: {}.", e).red()),
            },
            Command::BoardList(list_name) => {
                let lines = board.list_tasks(&list_name);
                if lines.is_empty() {
                    println!("{}", format!("No tasks in list '{}'.", list_name).yellow());
                } else {
                    TodoList::print_lines(lines);
                }
            }
            Command::BoardLists => {
                let names = board.list_names();
                if names.is_empty() {
                    println!("{}", "No lists yet.".yellow());
                } else {
                    println!(
                        "{}",
                        format!("Lists ({}): {}", names.len(), names.join(", ")).cyan()
                    );
                }
            }
            Command::BoardAssign(id, list_name) => {
                match board.assign_task_to_list(id, &list_name) {
                    Ok(()) => println!(
                        "{}",
                        format!("Task {} shared into list '{}'.", id, list_name).green()
                    ),
                    Err(e) => println!("{}", format!("Error: {}.", e).red()),
                }
            }
            Command::BoardUnassign(id, list_name) => match board.remove_from_list(id, &list_name) {
                Ok(()) => println!(
                    "{}",
                    format!("Task {} removed from list '{}'.", id, list_name).green()
                ),
                Err(e) => println!("{}", format!("Error: {}.", e).red()),
            },
            Command::BoardRemove(id) => match board.remove_task(id) {
                Ok(()) => println!(
                    "{}",
                    format!("Task {} removed entirely (all lists).", id).green()
                ),
                Err(e) => println!("{}", format!("Error: {}.", e).red()),
            },
            Command::BoardDeleteList(list_name) => match board.delete_list(&list_name) {
                Ok(()) => println!(
                    "{}",
                    format!(
                        "List '{}' deleted (its tasks were unassigned from it).",
                        list_name
                    )
                    .green()
                ),
                Err(e) => println!("{}", format!("Error: {}.", e).red()),
            },
            Command::BoardWhere(id) => {
                let lists = board.lists_containing(id);
                if lists.is_empty() {
                    println!("{}", format!("Task {} is not in any list.", id).yellow());
                } else {
                    println!(
                        "{}",
                        format!("Task {} is in: {}", id, lists.join(", ")).cyan()
                    );
                }
            }
        }
    }
}
