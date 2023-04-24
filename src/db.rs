use std::env;

use diesel::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

// connect to MySql DB
pub fn init_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    let conn_url = env::var("DATABASE_URL").expect("Failed to get value of DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(&conn_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
