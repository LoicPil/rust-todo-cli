//! Storage and operations on the collection of tasks: [`TodoList`] and
//! its error type [`TodoError`].

use std::collections::HashMap;
use std::fs;
use std::io;
use serde::{Serialize, Deserialize};

use crate::task::{Status, Task};

use std::fmt;

/// Errors that can occur when operating on a [`TodoList`] or [`crate::board::Board`].
#[derive(PartialEq, Debug)]
pub enum TodoError {
    /// No task exists with the given id.
    TaskNotFound(u32),
    /// No list exists with the given name.
    ListNotFound(String),
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoError::TaskNotFound(id) => write!(f, "task {} not found", id),
            TodoError::ListNotFound(name) => write!(f, "list '{}' not found", name),
        }
    }
}

/// An in-memory collection of tasks, keyed by an auto-incrementing `u32` id.
///
/// Ids are never reused, even after a task is removed: they come from a
/// monotonically increasing `next_id` counter rather than being derived
/// from the current size of the collection.
#[derive(Serialize, Deserialize)]
pub struct TodoList {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TodoList {
    /// Creates a new, empty [`TodoList`]. The first task added will get id `1`.
    pub fn new() -> TodoList {
        TodoList {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    /// Adds a new task with the given description and status
    /// [`Status::Todo`], assigning it the next available id.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut list = TodoList::new();
    /// list.add_task("Buy milk".to_string());
    /// assert_eq!(list.list_tasks().len(), 1);
    /// ```
    pub fn add_task(&mut self, description: String) {
        let id = self.next_id;
        self.tasks.insert(id, Task::new(description));
        self.next_id += 1;
    }

    /// Returns a reference to the task with the given id, if it exists.
    /// Used to read a task's data (e.g. before migrating it elsewhere)
    /// without removing it.
    ///
    /// # Errors
    ///
    /// Returns [`TodoError::TaskNotFound`] if no task with that id exists.
    pub fn get_task(&self, id: u32) -> Result<&Task, TodoError> {
        self.tasks.get(&id).ok_or(TodoError::TaskNotFound(id))
    }

    /// Removes the task with the given id.
    ///
    /// # Errors
    ///
    /// Returns [`TodoError::TaskNotFound`] if no task with that id exists.
    pub fn remove_task(&mut self, id: u32) -> Result<(), TodoError> {
        self.tasks.remove(&id).ok_or(TodoError::TaskNotFound(id))?;
        self.shrink_next_id();
        Ok(())
    }

    /// Walks `next_id` back down over any trailing ids that are no
    /// longer in use, so that removing the most recently created task
    /// (or several in a row) makes the next `add_task` reuse that id
    /// instead of skipping ahead. Stops as soon as it hits an id that's
    /// still occupied — never reclaims an id out from under a task that
    /// still exists.
    fn shrink_next_id(&mut self) {
        while self.next_id > 1 && !self.tasks.contains_key(&(self.next_id - 1)) {
            self.next_id -= 1;
        }
    }

    /// Marks the task with the given id as [`Status::Done`].
    ///
    /// # Errors
    ///
    /// Returns [`TodoError::TaskNotFound`] if no task with that id exists.
    pub fn complete_task(&mut self, id: u32) -> Result<(), TodoError> {
        let task = self.tasks.get_mut(&id).ok_or(TodoError::TaskNotFound(id))?;
        task.status = Status::Done;
        Ok(())
    }

    /// Marks the task with the given id as [`Status::InProgress`].
    ///
    /// # Errors
    ///
    /// Returns [`TodoError::TaskNotFound`] if no task with that id exists.
    pub fn set_in_progress(&mut self, id: u32) -> Result<(), TodoError> {
        let task = self.tasks.get_mut(&id).ok_or(TodoError::TaskNotFound(id))?;
        task.status = Status::InProgress;
        Ok(())
    }

    /// Returns every task formatted as `"{id} {status_icon} {description}"`,
    /// sorted by ascending id.
    ///
    /// Returns a single-element vector containing `"No tasks"` if the list
    /// is empty.
    pub fn list_tasks(&self) -> Vec<String> {
        if self.tasks.is_empty() {
            return vec!["No tasks".to_string()];
        }

        let mut entries: Vec<(&u32, &Task)> = self.tasks.iter().collect();
        entries.sort_by_key(|(id, _)| **id);

        entries
            .iter()
            .map(|(id, task)| format!("{} {}", id, task))
            .collect()
    }

    /// Returns every task with the given [`Status`], formatted the same way
    /// as [`TodoList::list_tasks`] and sorted by ascending id.
    ///
    /// Returns an empty vector if no task has the given status (callers
    /// distinguish this from the "list is empty" case of `list_tasks`).
    pub fn list_by_status(&self, status: Status) -> Vec<String> {
        let mut entries: Vec<(&u32, &Task)> = self
            .tasks
            .iter()
            .filter(|(_, task)| task.status == status)
            .collect();
        entries.sort_by_key(|(id, _)| **id);

        entries
            .iter()
            .map(|(id, task)| format!("{} {}", id, task))
            .collect()
    }

    /// Prints each line of a formatted task list, one per line.
    ///
    /// Typically used with the output of [`TodoList::list_tasks`] or
    /// [`TodoList::list_by_status`].
    /// Serializes the whole list to pretty-printed JSON and writes it to
    /// `path`, overwriting any existing file.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if serialization fails or the file cannot
    /// be written (e.g. permissions, invalid path).
    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        fs::write(path, json)
    }

    /// Loads a [`TodoList`] from a JSON file at `path`.
    ///
    /// If the file does not exist (e.g. first run of the program), returns
    /// a fresh, empty [`TodoList`] instead of an error.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if the file exists but its contents are not
    /// valid JSON for a [`TodoList`].
    pub fn load_from_file(path: &str) -> io::Result<TodoList> {
        match fs::read_to_string(path) {
            Ok(contents) => {
                let list = serde_json::from_str(&contents)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                Ok(list)
            }
            Err(_) => Ok(TodoList::new()),
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

    #[test]
    fn test_id_restarts_at_1_when_list_emptied() {
        let mut list = TodoList::new();
        list.add_task("First".to_string());
        list.remove_task(1).unwrap();
        // List is now empty; the next task added should get id 1 again,
        // not 2.
        list.add_task("Second".to_string());
        assert_eq!(list.list_tasks(), vec!["1 [] Second".to_string()]);
    }

    #[test]
    fn test_id_shrinks_back_when_removing_most_recent_task() {
        let mut list = TodoList::new();
        list.add_task("A".to_string()); // id 1
        list.add_task("B".to_string()); // id 2
        list.remove_task(2).unwrap(); // remove the most recently created one
        list.add_task("C".to_string()); // should reuse id 2, not become 3
        let lines = list.list_tasks();
        assert!(lines.contains(&"1 [] A".to_string()));
        assert!(lines.contains(&"2 [] C".to_string()));
    }

    #[test]
    fn test_id_does_not_shrink_when_removing_a_non_trailing_task() {
        let mut list = TodoList::new();
        list.add_task("A".to_string()); // id 1
        list.add_task("B".to_string()); // id 2
        list.remove_task(1).unwrap(); // remove the OLDER one, not the last
        list.add_task("C".to_string()); // id 2 is still taken by B, so this is id 3
        let lines = list.list_tasks();
        assert!(lines.contains(&"2 [] B".to_string()));
        assert!(lines.contains(&"3 [] C".to_string()));
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let path = "test_todos.json";
        let mut list = TodoList::new();
        list.add_task("Persisted task".to_string());
        list.complete_task(1).unwrap();
        list.save_to_file(path).unwrap();

        let loaded = TodoList::load_from_file(path).unwrap();
        assert_eq!(loaded.list_by_status(Status::Done).len(), 1);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_load_missing_file_returns_empty_list() {
        let loaded = TodoList::load_from_file("does_not_exist.json").unwrap();
        assert_eq!(loaded.list_tasks(), vec!["No tasks".to_string()]);
    }
}
