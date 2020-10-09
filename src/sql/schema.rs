//#[macro_use]
//extern crate diesel;

table! {
    use diesel::sql_types::*;

    ip (id) {
        id -> Integer,
	#[sql_name = "ip"]
        c_ip -> Text,
    }
}
table! {
    use diesel::sql_types::*;

    files (id) {
	id -> Integer,
	path -> Text,
	filename -> Text,
//	chdate -> Datetime,
	synced -> Bool,
	deleted -> Bool,
    }
}
