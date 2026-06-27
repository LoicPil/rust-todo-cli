use std::collections::HashMap;

use crate::task::{Status, Task};

pub enum TodoError {
    TaskNotFound(u32),
}

pub struct TodoList {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, description: String) {
        let id = self.next_id;
        self.tasks.insert(id, Task::new(description));
        self.next_id += 1;
    }

    pub fn remove_task(&mut self, id: u32) -> Result<(), TodoError> {
        self.tasks.remove(&id).ok_or(TodoError::TaskNotFound(id))?;
        Ok(())
    }
    pub fn complete_task(&mut self, id: u32) -> Result<(), TodoError> {
        let task = self.tasks.get_mut(&id).ok_or(TodoError::TaskNotFound(id))?;
        task.status = Status::Done;
        Ok(())
    }

    pub fn set_in_progress(&mut self, id: u32) -> Result<(), TodoError> {
        let task = self.tasks.get_mut(&id).ok_or(TodoError::TaskNotFound(id))?;
        task.status = Status::InProgress;
        Ok(())
    }

    pub fn list_tasks(&self) -> Vec<String> {
        if self.tasks.is_empty() {
            return vec!["No tasks".to_string()];
        }

        self.tasks
            .iter()
            .map(|(id, task)| format!("{} {}", id, task))
            .collect()
    }

    /// Retourne une liste de chaînes formatées filtrées par statut
    pub fn list_by_status(&self, status: Status) -> Vec<String> {
        self.tasks
            .iter()
            .filter(|(_, task)| task.status == status)
            .map(|(id, task)| format!("{} {}", id, task))
            .collect()
    }
    pub fn print_lines(lines: Vec<String>) {
        for line in lines {
            println!("{}", line);
        }
    }
}
