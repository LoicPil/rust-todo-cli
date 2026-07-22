//! Defines the core data model of the todo app: [`Status`] and [`Task`].

use serde::{Deserialize, Serialize};
use std::fmt;

/// The lifecycle state of a [`Task`].
///
/// A task always starts as `Todo` and moves to `InProgress` or `Done`
/// via [`crate::todo_list::TodoList::set_in_progress`] and
/// [`crate::todo_list::TodoList::complete_task`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Status {
    /// Not started yet. The default status for a newly created task.
    Todo,
    /// Currently being worked on.
    InProgress,
    /// Finished.
    Done,
}

/// A single todo item: a description plus its current [`Status`].
///
/// `Task` itself does not store an id — ids are assigned and owned by
/// [`crate::todo_list::TodoList`], which keeps tasks in a
/// `HashMap<u32, Task>`.
#[derive(Serialize, Deserialize)]
pub struct Task {
    /// Free-text description of what the task is.
    pub description: String,
    /// Current lifecycle state of the task.
    pub status: Status,
}

impl Task {
    /// Creates a new [`Task`] with the given description and status
    /// [`Status::Todo`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_todo_cli::task::Task;
    ///
    /// let task = Task::new("Buy milk".to_string());
    /// assert_eq!(task.description, "Buy milk");
    /// ```
    pub fn new(description: String) -> Task {
        Task {
            description,
            status: Status::Todo,
        }
    }
}

impl fmt::Display for Status {
    /// Renders the status as a short icon: `[]` for `Todo`, `[~]` for
    /// `InProgress`, `[x]` for `Done`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = match self {
            Status::Todo => "[]",
            Status::InProgress => "[~]",
            Status::Done => "[x]",
        };
        write!(f, "{}", icon)
    }
}

impl fmt::Display for Task {
    /// Renders a task as `"{status_icon} {description}"`,
    /// e.g. `"[~] Buy milk"`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.status, self.description)
    }
}
