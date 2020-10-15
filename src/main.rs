#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::json::Json;
use rocket_contrib::database;
use rocket_contrib::databases::diesel;

use rust_rest_api::*;
use rust_rest_api::models::*;

#[database("rust_rest_api")]
struct Connection(diesel::PgConnection);

#[post("/api/applications", data = "<input>")]
fn create_application(conn: Connection, input: Json<NewApplication>) -> Json<Application> {
    let application = insert_application(&conn, input.into_inner());

    let applicant = insert_applicant(&conn, NewApplicant {
        first_name: "John",
        lastname: "Smith",
        email: "john.smith@mailinator.com"
    });

    insert_application_applicant(&conn, NewApplicationsApplicant {
        application_id: application.id,
        applicant_id: applicant.id
    });

    Json(application)
}

#[put("/api/applications/<id>/name", data = "<input>")]
fn update_application_name(conn: Connection, id: i32, input: String) -> Json<Application> {
    let application = rust_rest_api::update_application_name(&conn, id, input);
    Json(application)
}

#[delete("/api/applications/<id>")]
fn delete_application(conn: Connection, id: i32) {
    rust_rest_api::delete_application(&conn, id)
}

#[get("/api/applications")]
fn read_applications(conn: Connection) -> Json<Vec<Application>> {
    let applications = rust_rest_api::read_applications(&conn);
    Json(applications)
}

#[get("/api/applications/<id>")]
fn read_application(conn: Connection, id: i32) -> Json<Application> {
    let application = rust_rest_api::read_application(&conn, id);
    Json(application)
}

#[get("/api/applications/<id>?relations=true")]
fn read_application_with_relations(conn: Connection, id: i32) -> Json<ApplicationWithRelations> {
    let application = rust_rest_api::read_application_with_relations(&conn, id);
    Json(application)
}

#[post("/api/applications/<id>/files", data = "<input>")]
fn create_file(conn: Connection, id: i32, input: Json<NewFile>) -> Json<File> {
    let file = insert_file(&conn, id, input.into_inner());
    Json(file)
}

#[get("/api/applications/<id>/files")]
fn read_files(conn: Connection, id: i32) -> Json<Vec<File>> {
    let files = rust_rest_api::read_files(&conn, id);
    Json(files)
}

#[post("/api/applications/<id>/applicants", data = "<input>")]
fn create_applicant(conn: Connection, id: i32, input: Json<NewApplicant>) -> Json<Applicant> {
    let applicant = insert_applicant(&conn, input.into_inner());

    insert_application_applicant(&conn, NewApplicationsApplicant {
        application_id: id,
        applicant_id: applicant.id
    });

    Json(applicant)
}

#[delete("/api/applicants/<id>")]
fn delete_applicant(conn: Connection, id: i32) {
    rust_rest_api::delete_applicant(&conn, id)
}

#[get("/api/applicants")]
fn read_applicants(conn: Connection) -> Json<Vec<Applicant>> {
    let applicants = rust_rest_api::read_applicants(&conn);
    Json(applicants)
}

#[get("/api/applicants/<id>")]
fn read_applicant(conn: Connection, id: i32) -> Json<Applicant> {
    let applicant = rust_rest_api::read_applicant(&conn, id);
    Json(applicant)
}

fn main() {
    rocket::ignite()
        .attach(Connection::fairing())
        .mount("/",
            routes![
                create_application,
                read_applications,
                read_application,
                read_application_with_relations,
                update_application_name,
                delete_application,
                create_file,
                read_files,
                create_applicant,
                read_applicants,
                read_applicant,
                delete_applicant,
            ]
        )
        .launch();
}