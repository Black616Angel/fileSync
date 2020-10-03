use self::models::*;
use diesel::prelude::*;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

pub fn get_ip() -> String {
    use self::schema::ip::dsl::*;
    let conn = ip_connection();
    let results = ip.filter(id.eq(1))
	.load::<Ip>(&conn)
	.expect("FEHLER");
    for line in results {
	return line.c_ip;
    }
    return "".to_string();
}

fn files_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_FILES")
	.expect("FILE_URL not set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn ip_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
