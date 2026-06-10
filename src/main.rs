use std::io;

struct Task {
    description: String,
    done: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            done: false,
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
        self.tasks.remove(index);
    }

    fn complete_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.done = true;
        } else {
            println!("Invalid task include!")
        }
    }

    fn list_tasks(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            let status: &str = if task.done { "[X]" } else { "[ ]" };
            println!("{}{}{}", i, status, task.description);
        }
    }
}

fn main() {
    let mut todo = TodoList::new();
    loop {
        println!("Commands: add <desc> | list | done <id> | remove <id> | quit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        let mut parts = input.splitn(2, ' ');
        let command = parts.next().unwrap_or("");
        let argument = parts.next().unwrap_or("");

        if command == "add" {
            todo.add_task(String::from(argument));
        } else if command == "list" {
            todo.list_tasks();
        } else if command == "done" {
            match argument.parse::<usize>() {
                Ok(index) => todo.complete_task(index),
                Err(_) => println!("Please provide a valid number"),
            }
        } else if command == "remove" {
            match argument.parse::<usize>() {
                Ok(index) => todo.remove_task(index),
                Err(_) => println!("Please provide a valid number"),
            }
        } else if command == "quit" {
            break;
        } else {
            println!("Unknown command !")
        }
        println!("----------");
        todo.list_tasks();
        println!("----------");
    }
}
