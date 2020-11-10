use super::schema::files;
use serde::Deserialize;

#[derive(Queryable)]
pub struct Ip {
    pub id: i32,
    pub c_ip: String,
}

#[derive(Queryable, Debug, Deserialize, Clone)]
//#[table_name="files"]
pub struct File {
    pub id: i32,
    pub path: String,
    pub filename: String,
    pub chdate: chrono::NaiveDateTime,
    pub synced: bool,
    pub deleted: bool,
}

#[derive(Insertable, Deserialize, Clone)]
#[table_name="files"]
pub struct NFile {
	pub path: String,
	pub filename: String,
	pub chdate: chrono::NaiveDateTime,
}

impl File {
    pub fn to_nfile(self: File) -> NFile {
        NFile { path : self.path, filename : self.filename, chdate: self.chdate}
    }
}

impl NFile {
    pub fn to_file(self: NFile) -> File {
        File { path : self.path, filename : self.filename, chdate: self.chdate, synced: false, deleted: false, id: 0}
    }
}
