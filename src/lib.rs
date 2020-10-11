#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use models::*;

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
    	.expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn insert_application(conn: &PgConnection, input: NewApplication) -> Application {
    use schema::applications;

    diesel::insert_into(applications::table)
        .values(input)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn update_application_name_(conn: &PgConnection, a_id: i32, input: String) -> Application {
    use schema::applications::dsl::*;

    diesel::update(applications.filter(id.eq(a_id)))
        .set(name.eq(input))
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn delete_application_(conn: &PgConnection, a_id: i32) {
    use schema::applications::dsl::*;

    diesel::delete(applications.filter(id.eq(a_id)))
        .execute(conn)
        .expect("Error deleting application");
}

pub fn read_applications_(conn: &PgConnection) -> Vec<Application> {
    use schema::applications::dsl::*;

    applications
        .load::<Application>(conn)
        .expect("Error getting applications")
}

pub fn read_application_(conn: &PgConnection, a_id: i32) -> Application {
    use schema::applications::dsl::*;

    applications.filter(id.eq(a_id))
        .get_result(conn)
        .expect("Error getting application")
}

pub fn read_application_with_relations_(conn: &PgConnection, a_id: i32) -> ApplicationWithRelations {
    use schema::applications::dsl::*;

    let application: Application = applications
        .filter(id.eq(a_id))
        .first(conn)
        .expect("Error getting application");
    let files: Vec<File> = File::belonging_to(&application)
        .load::<File>(conn)
        .expect("Error getting files");

    to_application_with_relations(application, files)
}

pub fn insert_file(conn: &PgConnection, _a_id: i32, input: NewFile) -> File {
    use schema::files;

    diesel::insert_into(files::table)
        .values(input)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn read_files_(conn: &PgConnection, a_id: i32) -> Vec<File> {
    use schema::files::dsl::*;

    files.filter(application_id.eq(a_id))
        .get_results(conn)
        .expect("Error getting files")
}

pub fn to_application_with_relations(application: Application, files: Vec<File>) -> ApplicationWithRelations {
    ApplicationWithRelations {
        application: application,
        files: to_files_with_relations(files)
    }
}

pub fn to_files_with_relations(files: Vec<File>) -> Vec<FileWithRelations> {
    files
        .into_iter()
        .map(|file| FileWithRelations { file })
        .collect::<Vec<_>>()
}
