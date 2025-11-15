use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::{storage};

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
    let mut tasks = list();
    let last_task = tasks.get(tasks.len() - 1).unwrap();

    let task = Task {
        id: last_task.id + 1,
        title: title,
        status: Status::ToDo,
        created_at: current_time(),
    };

    tasks.push(task);

    storage::write(tasks);
}

pub fn remove(id: i32) {
    let mut tasks = list();

    tasks = tasks.into_iter().filter(|task| task.id != id).collect();

    storage::write(tasks);
}

pub fn update(id: i32, title: String) {
    let mut tasks = list();

    let task_to_update = tasks.iter_mut().find(|t| t.id == id).unwrap();
    task_to_update.title = title;

    storage::write(tasks);
}
