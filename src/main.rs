use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use actix_web::{ get, post, App, Responder, HttpResponse, HttpServer, web };

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u64,
    name: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String, //can use other data types for better security
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    tasks: HashMap<u64, Task>, // hashmap is used as it can be converted to JSON easily
    users: HashMap<u64, User>,
}

impl Database {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }

    // CRUD task operation
    fn upsert(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    fn get(&self, task_id: &u64) -> Option<&Task> {
        self.tasks.get(task_id)
    }

    fn get_all(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    fn delete(&mut self, task_id: &u64) {
        self.tasks.remove(task_id);
    }
}

fn main() {
    println!("Hello, world!");
}
