#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;

use rocket_contrib::json::Json;
use diesel::prelude::*;

use rust_rest_api::*;
use rust_rest_api::models::*;
use rust_rest_api::schema::applications::dsl::*;

#[post("/api/applications", data = "<input>")]
fn create(input: Json<NewApplication>) -> Json<Application> {
    use schema::applications;

    let connection = establish_connection();
    let application = diesel::insert_into(applications::table)
        .values(input.into_inner())
        .get_result(&connection)
        .expect("Error saving new post");
    Json(application)
}

#[put("/api/applications/<_id>/name", data = "<input>")]
fn update_name(_id: i32, input: String) -> Json<Application> {
    let connection = establish_connection();
    let application = diesel::update(applications.filter(id.eq(_id)))
        .set(name.eq(input))
        .get_result(&connection)
        .expect("Error saving new post");
    Json(application)
}

#[delete("/api/applications/<_id>")]
fn delete(_id: i32) {
    let connection = establish_connection();
    diesel::delete(applications.filter(id.eq(_id)))
        .execute(&connection)
        .expect("Error deleting application");
}

#[get("/api/applications")]
fn read_all() -> Json<Vec<Application>> {
    let connection = establish_connection();
    let results = applications
        .load::<Application>(&connection)
        .expect("Error getting applications");
    Json(results)
}

#[get("/api/applications/<_id>")]
fn read_one(_id: i32) -> Json<Application> {
    let connection = establish_connection();
    let result = applications.filter(id.eq(_id))
        .get_result(&connection)
        .expect("Error getting application");
    Json(result)
}

fn main() {
    rocket::ignite()
        .mount("/",
            routes![
                create,
                read_all,
                read_one,
                update_name,
                delete
            ]
        )
        .launch();
}