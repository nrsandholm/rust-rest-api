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