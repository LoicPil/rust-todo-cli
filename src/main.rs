use std::io;

#[derive(Clone, Copy)]
enum Status {
    Done,
    Todo,
    InProgress,
}

struct Task {
    description: String,
    status: Status,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            status: Status::Todo,
        }
    }
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        self.tasks.push(Task::new(description));
    }

    fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
        } else {
            println!("Invalid task index");
        }
    }

    fn complete_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.status = Status::Done;
        } else {
            println!("Invalid task index");
        }
    }

    fn set_in_progress(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.status = Status::InProgress;
        } else {
            println!("Invalid task index");
        }
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks");
            return;
        }

        for (i, task) in self.tasks.iter().enumerate() {
            let status = match task.status {
                Status::Done => "[X]",
                Status::Todo => "[ ]",
                Status::InProgress => "[~]",
            };

            println!("{} {} {}", i, status, task.description);
        }
    }
}

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
