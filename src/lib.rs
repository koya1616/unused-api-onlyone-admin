pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("POSTGRESQL_DATABASE_URL").expect("POSTGRESQL_DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}