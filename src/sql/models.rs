#[derive(Queryable)]
pub struct Ip {
    pub id: i32,
    pub c_ip: String,
}

pub struct File {
    pub id: i32,
    pub path: String,
    pub filename: String,
    pub synced: bool,
    pub deleted: bool,
}
