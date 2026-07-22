use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub status: Status,
}

impl Task {
    pub fn new(description: String) -> Task {
        Task {
            description,
            status: Status::Todo,
        }
    }
}

impl fmt::Display for Status {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.status, self.description)
    }
}
