//use std::time::SystemTime;
//use chrono;

#[derive(Queryable)]
pub struct Ip {
    pub id: i32,
    pub c_ip: String,
}

#[derive(Queryable, Debug, Clone)]
pub struct File {
    pub id: i32,
    pub path: String,
    pub filename: String,
//    pub chdate: chrono::NaiveDateTime,
    pub synced: bool,
    pub deleted: bool,
}
