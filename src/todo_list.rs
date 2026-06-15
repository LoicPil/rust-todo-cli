use crate::task::{Status, Task};

pub struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, description: String) {
        self.tasks.push(Task::new(description));
    }

    pub fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
        } else {
            println!("Invalid task index");
        }
    }

    pub fn complete_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.status = Status::Done;
        } else {
            println!("Invalid task index");
        }
    }

    pub fn set_in_progress(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.status = Status::InProgress;
        } else {
            println!("Invalid task index");
        }
    }

    pub fn list_tasks(&self) {
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
