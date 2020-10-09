use serde::{Serialize, Deserialize};
use super::schema::applications;
use super::schema::files;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Application {
	pub id: i32,
	pub name: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="applications"]
pub struct NewApplication {
    pub name: String
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
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
