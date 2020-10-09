#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::json::Json;
use rocket_contrib::database;
use rocket_contrib::databases::diesel;

use diesel::prelude::*;

use rust_rest_api::*;
use rust_rest_api::models::*;

#[database("rust_rest_api")]
struct Connection(diesel::PgConnection);

#[post("/api/applications", data = "<input>")]
fn create_application(conn: Connection, input: Json<NewApplication>) -> Json<Application> {
    use schema::applications;

    let application = diesel::insert_into(applications::table)
        .values(input.into_inner())
        .get_result(&*conn)
        .expect("Error saving new post");
    Json(application)
}

#[put("/api/applications/<a_id>/name", data = "<input>")]
fn update_application_name(conn: Connection, a_id: i32, input: String) -> Json<Application> {
    use schema::applications::dsl::*;

    let application = diesel::update(applications.filter(id.eq(a_id)))
        .set(name.eq(input))
        .get_result(&*conn)
        .expect("Error saving new post");
    Json(application)
}

#[delete("/api/applications/<a_id>")]
fn delete_application(conn: Connection, a_id: i32) {
    use schema::applications::dsl::*;

    diesel::delete(applications.filter(id.eq(a_id)))
        .execute(&*conn)
        .expect("Error deleting application");
}

#[get("/api/applications")]
fn read_applications(conn: Connection) -> Json<Vec<Application>> {
    use schema::applications::dsl::*;

    let results = applications
        .load::<Application>(&*conn)
        .expect("Error getting applications");
    Json(results)
}

#[get("/api/applications/<a_id>")]
fn read_application(conn: Connection, a_id: i32) -> Json<Application> {
    use schema::applications::dsl::*;

    let result = applications.filter(id.eq(a_id))
        .get_result(&*conn)
        .expect("Error getting application");
    Json(result)
}

#[post("/api/applications/<_a_id>/files", data = "<input>")]
fn create_file(conn: Connection, _a_id: i32, input: Json<NewFile>) -> Json<File> {
    use schema::files;

    let file = diesel::insert_into(files::table)
        .values(input.into_inner())
        .get_result(&*conn)
        .expect("Error saving new post");
    Json(file)
}

#[get("/api/applications/<a_id>/files")]
fn read_files(conn: Connection, a_id: i32) -> Json<Vec<File>> {
    use schema::files::dsl::*;

    let results = files.filter(application_id.eq(a_id))
        .get_results(&*conn)
        .expect("Error getting files");
    Json(results)
}

fn main() {
    rocket::ignite()
        .attach(Connection::fairing())
        .mount("/",
            routes![
                create_application,
                read_applications,
                read_application,
                update_application_name,
                delete_application,
                create_file,
                read_files
            ]
        )
        .launch();
}