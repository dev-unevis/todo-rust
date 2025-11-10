use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::storage;

// Provide default for created_at
fn current_time() -> DateTime<Local> {
    Local::now()
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    ToDo,
    InProgress,
    Completed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: i32,
    title: String,
    status: Status,
    #[serde(default = "current_time")]
    created_at: DateTime<Local>,
}

pub fn list() -> Vec<Task> {
    storage::read()
}

pub fn add(title: String) {
    let task = Task {
        id: 2,
        title: title,
        status: Status::ToDo,
        created_at: current_time(),
    };

    let mut tasks = list();

    tasks.push(task);

    storage::write(tasks);
}
