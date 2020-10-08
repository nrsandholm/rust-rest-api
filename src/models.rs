use serde::{Serialize, Deserialize};
use super::schema::applications;

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