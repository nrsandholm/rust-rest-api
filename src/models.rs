use serde::{Serialize, Deserialize};
use super::schema::applications;
use super::schema::files;
use super::schema::applicants;
use super::schema::applications_applicants;

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Application {
	pub id: i32,
	pub name: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="applications"]
pub struct NewApplication {
    pub name: String
}

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable, Associations)]
#[belongs_to(Application)]
pub struct File {
	pub id: i32,
	pub application_id: i32,
	pub uri: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="files"]
pub struct NewFile {
    pub application_id: i32,
    pub uri: String,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Applicant {
	pub id: i32,
	pub first_name: String,
	pub lastname: String,
	pub email: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="applicants"]
pub struct NewApplicant<'a> {
    pub first_name: &'a str,
	pub lastname: &'a str,
	pub email: &'a str
}

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable, Associations)]
#[belongs_to(Application)]
#[belongs_to(Applicant)]
pub struct ApplicationsApplicant {
	pub id: i32,
	pub application_id: i32,
	pub applicant_id: i32
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="applications_applicants"]
pub struct NewApplicationsApplicant {
	pub application_id: i32,
	pub applicant_id: i32
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationWithRelations {
	#[serde(flatten)]
	pub application: Application,
	pub applicants: Vec<ApplicantWithRelations>,
	pub files: Vec<FileWithRelations>
}

#[derive(Serialize, Deserialize)]
pub struct ApplicantWithRelations {
	#[serde(flatten)]
	pub applicant: Applicant
}

#[derive(Serialize, Deserialize)]
pub struct FileWithRelations {
	#[serde(flatten)]
	pub file: File
}