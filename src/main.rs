#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use serde::{Serialize, Deserialize};
use rocket_contrib::json::Json;
use rocket::State;
use std::sync::Mutex;

struct Database {
	items: Mutex<Vec<Application>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Application {
	id: String,
	name: String
}

#[post("/api/applications", data = "<input>")]
fn create(input: Json<Application>, db: State<Database>) -> Json<Application> {
	let mut items = db.items.lock().expect("lock shared data");
	let application: Application = input.into_inner();
    items.push(application.clone());
    Json(application)
}

#[delete("/api/applications/<id>")]
fn delete(id: String, db: State<Database>) {
	let mut items = db.items.lock().expect("lock shared data");
    items.retain(|i| i.id != id);
}

#[get("/api/applications")]
fn read_all(db: State<Database>) -> Json<Vec<Application>> {
	let items = db.items.lock().expect("lock shared data");
    Json(items.to_vec())
}

fn main() {
    rocket::ignite()
    	.manage(Database { items: Mutex::new(Vec::new()) })
    	.mount("/", 
    		routes![
    			create,
    			read_all,
    			delete
    		]
    	)
    	.launch();
}