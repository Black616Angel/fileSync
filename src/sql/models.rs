//use std::time::SystemTime;
//use chrono;
//#[path="schema.rs"] mod schema;
use super::schema::files;

#[derive(Queryable)]
pub struct Ip {
    pub id: i32,
    pub c_ip: String,
}

#[derive(Queryable, Debug, Clone)]
//#[table_name="files"]
pub struct File {
    pub id: i32,
    pub path: String,
    pub filename: String,
//    pub chdate: chrono::NaiveDateTime,
    pub synced: bool,
    pub deleted: bool,
}

#[derive(Insertable, Clone)]
#[table_name="files"]
pub struct NFile {
	pub path: String,
	pub filename: String,
}

