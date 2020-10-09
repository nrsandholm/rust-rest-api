#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::json::Json;
use rocket_contrib::database;
use rocket_contrib::databases::diesel;

use diesel::prelude::*;

use rust_rest_api::*;
use rust_rest_api::models::*;
use rust_rest_api::schema::applications::dsl::*;

#[database("rust_rest_api")]
struct Connection(diesel::PgConnection);

#[post("/api/applications", data = "<input>")]
fn create(conn: Connection, input: Json<NewApplication>) -> Json<Application> {
    use schema::applications;

    let application = diesel::insert_into(applications::table)
        .values(input.into_inner())
        .get_result(&*conn)
        .expect("Error saving new post");
    Json(application)
}

#[put("/api/applications/<_id>/name", data = "<input>")]
fn update_name(conn: Connection, _id: i32, input: String) -> Json<Application> {
    let application = diesel::update(applications.filter(id.eq(_id)))
        .set(name.eq(input))
        .get_result(&*conn)
        .expect("Error saving new post");
    Json(application)
}

#[delete("/api/applications/<_id>")]
fn delete(conn: Connection, _id: i32) {
    diesel::delete(applications.filter(id.eq(_id)))
        .execute(&*conn)
        .expect("Error deleting application");
}

#[get("/api/applications")]
fn read_all(conn: Connection) -> Json<Vec<Application>> {
    let results = applications
        .load::<Application>(&*conn)
        .expect("Error getting applications");
    Json(results)
}

#[get("/api/applications/<_id>")]
fn read_one(conn: Connection, _id: i32) -> Json<Application> {
    let result = applications.filter(id.eq(_id))
        .get_result(&*conn)
        .expect("Error getting application");
    Json(result)
}

fn main() {
    rocket::ignite()
        .attach(Connection::fairing())
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