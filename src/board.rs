//! Multiple named lists sharing tasks via `Rc<RefCell<Task>>` (Chapter 15).
//!
//! Unlike [`crate::todo_list::TodoList`], where each task belongs to
//! exactly one list, a [`Board`] holds one central registry of tasks and
//! lets several named lists reference the *same* task by id. Completing
//! a task from one list is visible from every other list it belongs to.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::rc::Rc;

use serde::{Serialize, Deserialize};

use crate::task::{Status, Task};
use crate::todo_list::TodoError;

/// A task that can have multiple owners (multiple lists pointing to it).
pub type SharedTask = Rc<RefCell<Task>>;

/// Central registry of tasks, plus named lists that each reference a
/// subset of those tasks by id.
pub struct Board {
    tasks: HashMap<u32, SharedTask>,
    /// Maps a list name (e.g. "Work") to the set of task ids it contains.
    /// `HashSet` (not `Vec`) because a task shouldn't appear twice in the
    /// same list, and we don't care about order here.
    lists: HashMap<String, HashSet<u32>>,
    next_id: u32,
}

/// Plain, serializable mirror of [`Board`]'s data, used only for saving
/// to and loading from disk.
///
/// `Rc<RefCell<Task>>` can't derive `Serialize`/`Deserialize` directly, so
/// on save we unwrap each task down to a plain [`Task`], and on load we
/// re-wrap each one in a fresh `Rc<RefCell<_>>`. Because sharing between
/// lists is implemented via task *ids* (both lists just store the same
/// `u32` in their `HashSet`), this round-trip fully preserves which lists
/// share which tasks — nothing is lost by dropping down to plain `Task`
/// for the JSON file.
#[derive(Serialize, Deserialize)]
struct BoardData {
    tasks: HashMap<u32, Task>,
    lists: HashMap<String, HashSet<u32>>,
    next_id: u32,
}

impl Board {
    pub fn new() -> Board {
        Board {
            tasks: HashMap::new(),
            lists: HashMap::new(),
            next_id: 1,
        }
    }

    /// Creates a new empty named list if one doesn't already exist.
    pub fn create_list(&mut self, name: &str) {
        self.lists.entry(name.to_string()).or_insert_with(HashSet::new);
    }

    /// Adds a brand-new task to the central registry and to the given
    /// list. Creates the list if it doesn't exist yet. Returns the new
    /// task's id.
    pub fn add_task(&mut self, list_name: &str, description: String) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let task = Rc::new(RefCell::new(Task::new(description)));
        self.tasks.insert(id, task);

        self.lists
            .entry(list_name.to_string())
            .or_insert_with(HashSet::new)
            .insert(id);

        id
    }

    /// Adds an *existing* task (by id) to another list, without
    /// duplicating it. This is the actual "sharing" step: the same
    /// `Rc<RefCell<Task>>` ends up referenced from two lists.
    ///
    /// # Errors
    ///
    /// Returns [`TodoError::TaskNotFound`] if `task_id` doesn't exist.
    pub fn assign_task_to_list(&mut self, task_id: u32, list_name: &str) -> Result<(), TodoError> {
        if !self.tasks.contains_key(&task_id) {
            return Err(TodoError::TaskNotFound(task_id));
        }

        self.lists
            .entry(list_name.to_string())
            .or_insert_with(HashSet::new)
            .insert(task_id);

        Ok(())
    }

    /// Marks a task done, wherever it's referenced from. Takes only an
    /// id (no list name) because the task is the same object regardless
    /// of which list you look at it through.
    ///
    /// # Errors
    ///
    /// Returns [`TodoError::TaskNotFound`] if `task_id` doesn't exist.
    pub fn complete_task(&mut self, task_id: u32) -> Result<(), TodoError> {
        let shared = self.tasks.get(&task_id).ok_or(TodoError::TaskNotFound(task_id))?;
        // borrow_mut() gives us temporary mutable access to the Task
        // inside the RefCell. It ends automatically when `task` goes
        // out of scope at the end of this block.
        let mut task = shared.borrow_mut();
        task.status = Status::Done;
        Ok(())
    }

    /// Formats every task in the given list as `"{id} {task}"`.
    /// Returns an empty vector if the list doesn't exist or is empty.
    pub fn list_tasks(&self, list_name: &str) -> Vec<String> {
        let Some(ids) = self.lists.get(list_name) else {
            return Vec::new();
        };

        let mut entries: Vec<(&u32, &SharedTask)> = ids
            .iter()
            .filter_map(|id| self.tasks.get(id).map(|task| (id, task)))
            .collect();
        entries.sort_by_key(|(id, _)| **id);

        entries
            .iter()
            .map(|(id, task)| {
                // borrow() gives read-only access; multiple borrow()s
                // can coexist, unlike borrow_mut().
                format!("{} {}", id, task.borrow())
            })
            .collect()
    }

    /// How many lists currently reference the given task id. Useful to
    /// see [`Rc`] sharing in action: this equals `Rc::strong_count`
    /// minus 1 (the `-1` is for the registry's own copy in `self.tasks`).
    pub fn lists_containing(&self, task_id: u32) -> Vec<String> {
        self.lists
            .iter()
            .filter(|(_, ids)| ids.contains(&task_id))
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// Serializes the board to pretty-printed JSON and writes it to
    /// `path`, overwriting any existing file.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if serialization fails or the file cannot
    /// be written.
    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
        // Unwrap each Rc<RefCell<Task>> down to a plain, owned Task by
        // cloning what's inside. borrow() is enough since we're only
        // reading.
        let tasks: HashMap<u32, Task> = self
            .tasks
            .iter()
            .map(|(id, shared)| (*id, shared.borrow().clone()))
            .collect();

        let data = BoardData {
            tasks,
            lists: self.lists.clone(),
            next_id: self.next_id,
        };

        let json = serde_json::to_string_pretty(&data)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        fs::write(path, json)
    }

    /// Loads a [`Board`] from a JSON file at `path`.
    ///
    /// If the file does not exist yet, returns a fresh, empty [`Board`]
    /// instead of an error.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if the file exists but its contents are
    /// not valid JSON for a [`Board`].
    pub fn load_from_file(path: &str) -> io::Result<Board> {
        let data: BoardData = match fs::read_to_string(path) {
            Ok(contents) => serde_json::from_str(&contents)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,
            Err(_) => {
                return Ok(Board::new());
            }
        };

        // Re-wrap each plain Task in a fresh Rc<RefCell<_>>. Two lists
        // that shared a task before saving still share it after loading,
        // because both of their HashSet<u32> entries point at the same
        // id, and we only create ONE Rc per id here.
        let tasks: HashMap<u32, SharedTask> = data
            .tasks
            .into_iter()
            .map(|(id, task)| (id, Rc::new(RefCell::new(task))))
            .collect();

        Ok(Board {
            tasks,
            lists: data.lists,
            next_id: data.next_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task_creates_list_and_task() {
        let mut board = Board::new();
        let id = board.add_task("Work", "Write report".to_string());
        assert_eq!(board.list_tasks("Work"), vec![format!("{} [] Write report", id)]);
    }

    #[test]
    fn test_shared_task_completed_in_one_list_shows_in_other() {
        let mut board = Board::new();
        let id = board.add_task("Work", "Shared task".to_string());
        board.assign_task_to_list(id, "Personal").unwrap();

        board.complete_task(id).unwrap();

        // Same underlying Rc<RefCell<Task>>, so both lists see it as Done.
        assert!(board.list_tasks("Work")[0].contains("[x]"));
        assert!(board.list_tasks("Personal")[0].contains("[x]"));
    }

    #[test]
    fn test_complete_task_not_found() {
        let mut board = Board::new();
        assert_eq!(board.complete_task(99), Err(TodoError::TaskNotFound(99)));
    }

    #[test]
    fn test_save_load_preserves_sharing() {
        let path = "test_board.json";
        let mut board = Board::new();
        let id = board.add_task("Work", "Shared task".to_string());
        board.assign_task_to_list(id, "Personal").unwrap();
        board.save_to_file(path).unwrap();

        let mut loaded = Board::load_from_file(path).unwrap();
        loaded.complete_task(id).unwrap();

        // Still shared after the round-trip: completing it once shows
        // up in both lists.
        assert!(loaded.list_tasks("Work")[0].contains("[x]"));
        assert!(loaded.list_tasks("Personal")[0].contains("[x]"));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_load_missing_file_returns_empty_board() {
        let board = Board::load_from_file("does_not_exist_board.json").unwrap();
        assert_eq!(board.list_tasks("Work"), Vec::<String>::new());
    }

    #[test]
    fn test_lists_containing() {
        let mut board = Board::new();
        let id = board.add_task("Work", "Shared task".to_string());
        board.assign_task_to_list(id, "Personal").unwrap();

        let mut lists = board.lists_containing(id);
        lists.sort();
        assert_eq!(lists, vec!["Personal".to_string(), "Work".to_string()]);
    }
}
