#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

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
