// 参考にした。https://planetscale.com/blog/build-a-rust-api-with-rocket-diesel-mysql
use std::env;

use diesel::prelude::*;
use dotenvy::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}