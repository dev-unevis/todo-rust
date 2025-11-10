use std::{fs, path::{Path, PathBuf}};

use crate::task::Task;

pub fn get_file_path() -> PathBuf {
    Path::new(file!()).parent().unwrap().join("tasks.json")
}

pub fn read() -> Vec<Task> {
    let tasks = fs::read_to_string(get_file_path()).unwrap_or_else(|_| "[]".to_string());

    let tasks_json = serde_json::from_str(&tasks).unwrap_or_else(|_| Vec::new());
    return tasks_json;
}

pub fn write(tasks: Vec<Task>) {
    let tasks_str = serde_json::to_string_pretty(&tasks).unwrap();
    let _ = fs::write(get_file_path(), tasks_str);
}
