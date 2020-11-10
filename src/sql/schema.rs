//#[macro_use]
//use diesel::data_types::*;
//use diesel::sql_types::*;

table! {
    ip (id) {
        id -> Integer,
	#[sql_name = "ip"]
        c_ip -> Text,
    }
}
table! {
    files (id) {
	id -> Integer,
	path -> Text,
	filename -> Text,
	chdate -> Timestamp,
	synced -> Bool,
	deleted -> Bool,
    }
}
