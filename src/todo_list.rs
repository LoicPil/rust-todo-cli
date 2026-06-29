use std::collections::HashMap;

use crate::task::{Status, Task};
#[derive(PartialEq, Debug)]
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut list = TodoList::new();
        list.add_task("Buy milk".to_string());
        assert_eq!(list.list_tasks().len(), 1);
    }

    #[test]
    fn test_complete_task() {
        let mut list = TodoList::new();
        list.add_task("Buy milk".to_string());
        assert_eq!(list.complete_task(1), Ok(()));
        assert_eq!(list.list_by_status(Status::Done).len(), 1);
    }

    #[test]
    fn test_complete_task_not_found() {
        let mut list = TodoList::new();
        assert_eq!(list.complete_task(99), Err(TodoError::TaskNotFound(99)));
    }

    #[test]
    fn test_remove_task() {
        let mut list = TodoList::new();
        list.add_task("Buy milk".to_string());
        assert_eq!(list.remove_task(1), Ok(()));
        assert_eq!(list.list_tasks(), vec!["No tasks".to_string()]);
    }

    #[test]
    fn test_remove_task_not_found() {
        let mut list = TodoList::new();
        assert_eq!(list.remove_task(99), Err(TodoError::TaskNotFound(99)));
    }

    #[test]
    fn test_set_in_progress() {
        let mut list = TodoList::new();
        list.add_task("Buy milk".to_string());
        assert_eq!(list.set_in_progress(1), Ok(()));
        assert_eq!(list.list_by_status(Status::InProgress).len(), 1);
    }

    #[test]
    fn test_filter_by_status() {
        let mut list = TodoList::new();
        list.add_task("Task A".to_string());
        list.add_task("Task B".to_string());
        list.complete_task(1).unwrap();
        assert_eq!(list.list_by_status(Status::Done).len(), 1);
        assert_eq!(list.list_by_status(Status::Todo).len(), 1);
    }
}
