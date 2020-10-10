use serde::{Serialize, Deserialize};
use super::schema::applications;
use super::schema::files;

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

#[derive(Serialize, Deserialize)]
pub struct ApplicationWithRelations {
	#[serde(flatten)]
	pub application: Application,
	pub files: Vec<FileWithRelations>
}

#[derive(Serialize, Deserialize)]
pub struct FileWithRelations {
	#[serde(flatten)]
	pub file: File
}