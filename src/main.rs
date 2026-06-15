mod task;
mod todo_list;

use crate::todo_list::TodoList;
use std::io;

enum Command {
    Add(String),
    Done(usize),
    Remove(usize),
    Progress(usize),
    List,
    Quit,
    Unknown,
}
fn parse_command(input: &str) -> Command {
    let mut parts = input.splitn(2, ' ');

    let command = parts.next().unwrap_or("");
    let argument = parts.next().unwrap_or("").trim();

    match command {
        "add" => Command::Add(argument.to_string()),

        "done" => match argument.parse::<usize>() {
            Ok(id) => Command::Done(id),
            Err(_) => Command::Unknown,
        },

        "remove" => match argument.parse::<usize>() {
            Ok(id) => Command::Remove(id),
            Err(_) => Command::Unknown,
        },

        "progress" => match argument.parse::<usize>() {
            Ok(id) => Command::Progress(id),
            Err(_) => Command::Unknown,
        },

        "list" => Command::List,
        "quit" => Command::Quit,

        _ => Command::Unknown,
    }
}
fn main() {
    let mut todo = TodoList::new();

    loop {
        println!("Commands: add <desc> | done <id> | remove <id> | progress <id> | list | quit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match parse_command(input.trim()) {
            Command::Add(desc) => todo.add_task(desc),
            Command::Done(id) => todo.complete_task(id),
            Command::Remove(id) => todo.remove_task(id),
            Command::Progress(id) => todo.set_in_progress(id),
            Command::List => todo.list_tasks(),
            Command::Quit => break,
            Command::Unknown => println!("Unknown command"),
        }
    }
}
