use std::env::{self};
use todo::task;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("list") => {
            let tasks = task::list();
            println!("{:?}", tasks)
        }
        Some("add") => {
            if let Some(todo_title) = args.get(2) {
                task::add(todo_title.to_string());
            } else {
                println!("Please insert a todo title!");
            }
        }
        Some("remove") => {
            if let Some(id) = args.get(2) {
                task::remove(id.parse().unwrap());
            }
        }
        Some("update") => {
            if let (Some(id), Some(title)) = (args.get(2), args.get(3)) {
                task::update(id.parse().unwrap(), title.to_string());
            }
        }
        _ => println!("Unknown command"),
    }
}
