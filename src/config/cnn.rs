extern crate rusqlite;
extern crate dotenv;

use rusqlite::Connection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> Connection {
    dotenv().ok();

    let database_path = env::var("DATABASE_PATH")
        .unwrap_or_else(|_| "./db/rusqlite.db".to_string());
    
    Connection::open(database_path)
        .expect("Error connecting to the database")
}
