use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
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
    fn upsert_task(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    fn get_task(&self, task_id: &u64) -> Option<&Task> {
        self.tasks.get(task_id)
    }

    fn get_all_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    fn delete_task(&mut self, task_id: &u64) {
        self.tasks.remove(task_id);
    }

    // User CRUD
    fn upsert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    fn get_user_by_name(&self, user_name: &str) -> Option<&User> {
        self.users.values().find(|user| user.username == user_name)
    }

    // Database methods
    fn save_to_file(&self) -> std::io::Result<()> {
        // convert the current database(self) to string type
        let db_string = serde_json::to_string(&self)?;

        // write the db_string to a file
        let mut file = fs::File::create("database.json")?;
        file.write_all(db_string.as_bytes()); //need to store the file in bytes

        Ok(())
    }

    fn read_from_file() -> std::io::Result<Self> {
        // reat content from the file "database.json"
        let file_content = fs::read_to_string("database.json")?;

        // convert the string to Database(Self) type
        let db: Self = serde_json::from_str(&file_content)?;

        Ok(db)
    }
}

fn main() {
    println!("Hello, world!");
}
