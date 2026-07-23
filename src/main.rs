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
        "  {:<28} show which lists contain a task",
        "bwhere <id>".magenta()
    );

    println!("\n{}", "── Other ──".bold());
    println!("  {:<28} show this help", "help / ?".yellow());
    println!("  {:<28} save and exit", "quit".yellow());
}

fn main() {
    let mut todo = TodoList::load_from_file(SAVE_PATH).unwrap_or_else(|_| TodoList::new());
    let mut board = Board::load_from_file(BOARD_SAVE_PATH).unwrap_or_else(|_| Board::new());

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
                Err(TodoError::TaskNotFound(id)) => {
                    println!("{}", format!("Error: task {} not found.", id).red())
                }
            },
            Command::Remove(id) => match todo.remove_task(id) {
                Ok(()) => println!("{}", format!("Task {} removed.", id).green()),
                Err(TodoError::TaskNotFound(id)) => {
                    println!("{}", format!("Error: task {} not found.", id).red())
                }
            },
            Command::Progress(id) => match todo.set_in_progress(id) {
                Ok(()) => println!("{}", format!("Task {} marked as in progress.", id).green()),
                Err(TodoError::TaskNotFound(id)) => {
                    println!("{}", format!("Error: task {} not found.", id).red())
                }
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
                if let Err(e) = todo.save_to_file(SAVE_PATH) {
                    println!("{}", format!("Warning: failed to save tasks: {}", e).red());
                }
                if let Err(e) = board.save_to_file(BOARD_SAVE_PATH) {
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
                Err(TodoError::TaskNotFound(id)) => {
                    println!("{}", format!("Error: task {} not found.", id).red())
                }
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
                    Err(TodoError::TaskNotFound(id)) => {
                        println!("{}", format!("Error: task {} not found.", id).red())
                    }
                }
            }
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

