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

#[put("/api/applications/<a_id>/name", data = "<input>")]
fn update_application_name(conn: Connection, a_id: i32, input: String) -> Json<Application> {
    let application = update_application_name_(&conn, a_id, input);
    Json(application)
}

#[delete("/api/applications/<a_id>")]
fn delete_application(conn: Connection, a_id: i32) {
    delete_application_(&conn, a_id)
}

#[get("/api/applications")]
fn read_applications(conn: Connection) -> Json<Vec<Application>> {
    let applications = read_applications_(&conn);
    Json(applications)
}

#[get("/api/applications/<a_id>")]
fn read_application(conn: Connection, a_id: i32) -> Json<Application> {
    let application = read_application_(&conn, a_id);
    Json(application)
}

#[get("/api/applications/<a_id>?relations=true")]
fn read_application_with_relations(conn: Connection, a_id: i32) -> Json<ApplicationWithRelations> {
    let application = read_application_with_relations_(&conn, a_id);
    Json(application)
}

#[post("/api/applications/<a_id>/files", data = "<input>")]
fn create_file(conn: Connection, a_id: i32, input: Json<NewFile>) -> Json<File> {
    let file = insert_file(&conn, a_id, input.into_inner());
    Json(file)
}

#[get("/api/applications/<a_id>/files")]
fn read_files(conn: Connection, a_id: i32) -> Json<Vec<File>> {
    let files = read_files_(&conn, a_id);
    Json(files)
}

#[post("/api/applicants", data = "<input>")]
fn create_applicant(conn: Connection, input: Json<NewApplicant>) -> Json<Applicant> {
    let applicant = insert_applicant(&conn, input.into_inner());
    Json(applicant)
}

#[delete("/api/applicants/<a_id>")]
fn delete_applicant(conn: Connection, a_id: i32) {
    delete_applicant_(&conn, a_id)
}

#[get("/api/applicants")]
fn read_applicants(conn: Connection) -> Json<Vec<Applicant>> {
    let applicants = read_applicants_(&conn);
    Json(applicants)
}

#[get("/api/applicants/<a_id>")]
fn read_applicant(conn: Connection, a_id: i32) -> Json<Applicant> {
    let applicant = read_applicant_(&conn, a_id);
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