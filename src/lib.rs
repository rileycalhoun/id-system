use std::env;

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;

extern crate serde_json;

pub mod commands;
pub mod console;
pub mod schema;
pub mod models;
pub mod state;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("Unable to find DATABASE URL in .env file");

    return PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
}
