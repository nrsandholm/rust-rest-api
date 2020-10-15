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
    let applicant_ids = read_application_applicant_by_application(conn, &application);
    let applicants = read_applicants(conn, &applicant_ids);

    to_application_with_relations(application, applicants, files)
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

pub fn insert_applicant(conn: &PgConnection, input: NewApplicant) -> Applicant {
    use schema::applicants;

    diesel::insert_into(applicants::table)
        .values(input)
        .get_result(conn)
        .expect("Error inserting applicant")
}

pub fn delete_applicant_(conn: &PgConnection, a_id: i32) {
    use schema::applicants::dsl::*;

    diesel::delete(applicants.filter(id.eq(a_id)))
        .execute(conn)
        .expect("Error deleting applicant");
}

pub fn read_applicants_(conn: &PgConnection) -> Vec<Applicant> {
    use schema::applicants::dsl::*;

    applicants
        .load::<Applicant>(conn)
        .expect("Error getting applicants")
}

pub fn read_applicant_(conn: &PgConnection, a_id: i32) -> Applicant {
    use schema::applicants::dsl::*;

    applicants.filter(id.eq(a_id))
        .get_result(conn)
        .expect("Error getting applicant")
}

fn read_applicants(conn: &PgConnection, applicant_ids: &Vec<i32>) -> Vec<Applicant> {
    use diesel::pg::expression::dsl::any;
    use schema::applicants::dsl::*;

    applicants.filter(id.eq(any(applicant_ids)))
        .load::<Applicant>(conn)
        .expect("Error getting applicants")
}

pub fn insert_application_applicant(conn: &PgConnection, input: NewApplicationsApplicant) -> ApplicationsApplicant {
    use schema::applications_applicants;

    diesel::insert_into(applications_applicants::table)
        .values(input)
        .get_result(conn)
        .expect("Error inserting applications applicant")
}

pub fn delete_application_applicant_(conn: &PgConnection, a_id: i32) {
    use schema::applications_applicants::dsl::*;

    diesel::delete(applications_applicants.filter(id.eq(a_id)))
        .execute(conn)
        .expect("Error deleting applications applicant");
}

fn read_application_applicant_by_application(conn: &PgConnection, application: &Application) -> Vec<i32> {
    use schema::applications_applicants;

    ApplicationsApplicant::belonging_to(application)
        .select(applications_applicants::applicant_id)
        .load::<i32>(conn)
        .expect("Error getting applications applicants")
}

pub fn to_application_with_relations(
        application: Application,
        applicants: Vec<Applicant>,
        files: Vec<File>
    ) -> ApplicationWithRelations {
    ApplicationWithRelations {
        application: application,
        applicants: to_applicants_with_relations(applicants),
        files: to_files_with_relations(files)
    }
}

pub fn to_applicants_with_relations(applicants: Vec<Applicant>) -> Vec<ApplicantWithRelations> {
    applicants
        .into_iter()
        .map(|applicant| ApplicantWithRelations { applicant })
        .collect::<Vec<_>>()
}

pub fn to_files_with_relations(files: Vec<File>) -> Vec<FileWithRelations> {
    files
        .into_iter()
        .map(|file| FileWithRelations { file })
        .collect::<Vec<_>>()
}
