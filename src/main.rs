use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{
    collections::HashMap,
    fs,
    sync::{Mutex, MutexGuard},
    vec,
};

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

    fn get_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    fn delete_user(&mut self, user_id: &u64) {
        self.users.remove(user_id);
    }

    // Database methods
    fn save_to_file(&self) -> std::io::Result<()> {
        // convert the current database(self) to string type
        let db_string = serde_json::to_string(&self)?;

        // write the db_string to a file
        let mut file = fs::File::create("database.json")?;
        let _ = file.write_all(db_string.as_bytes()); //need to store the file in bytes

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

struct AppState {
    db: Mutex<Database>,
}

async fn upsert_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    // lock or secure the app state(db)
    // instead of unwrap we could use expect for better error handling
    let mut db: MutexGuard<Database> = app_state.db.lock().unwrap();

    // insert task to the availed db
    // into_inner is used to unwrap the task from JSON
    db.upsert_task(task.into_inner());

    // save the updated db to our local database file "database.json"
    let _ = db.save_to_file();

    // complete the http response
    HttpResponse::Ok().finish()
}

async fn get_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db = app_state.db.lock().unwrap();

    match db.get_task(&id.into_inner()) {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn get_all_tasks(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().unwrap();

    let tasks = db.get_all_tasks();

    HttpResponse::Ok().json(tasks)
}

async fn delete_task(app_state: web::Data<AppState>, task_id: web::Path<u64>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();

    db.delete_task(&task_id.into_inner());

    let _ = db.save_to_file();

    HttpResponse::Ok().finish()
}

// USER api methods
async fn register(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();

    db.upsert_user(user.into_inner());

    let _ = db.save_to_file();

    HttpResponse::Ok().finish()
}

async fn login(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let db = app_state.db.lock().unwrap();

    match db.get_user_by_name(&user.username) {
        Some(searched_user) if searched_user.password == user.password => {
            HttpResponse::Ok().body("Logged In")
        }
        _ => HttpResponse::BadRequest().body("Invalid username or password"),
    }
}

async fn get_all_users(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().unwrap();

    let users = db.get_users();

    HttpResponse::Ok().json(users)
}

async fn delete_user(app_state: web::Data<AppState>, user_id: web::Path<u64>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();

    db.delete_user(&user_id.into_inner());

    let _ = db.save_to_file();

    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // fetch an existing database or create one
    let db: Database = match Database::read_from_file() {
        Ok(db) => db,
        Err(_) => Database::new(),
    };

    // create web app data
    let web_data = web::Data::new(AppState { db: Mutex::new(db) });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(web_data.clone()) //clones only the pointer the actual web_data
            .route("/task", web::post().to(upsert_task))
            .route("task/{id}", web::get().to(get_task))
            .route("task", web::get().to(get_all_tasks))
            .route("task/{id}", web::delete().to(delete_task))
            .route("/user", web::get().to(get_all_users))
            .route("/user/{id}", web::delete().to(delete_user))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
