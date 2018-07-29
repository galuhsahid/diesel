#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::Post;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &MysqlConnection, title: &str, body: &str) -> Post {
    use schema::posts;

    diesel::insert_into(posts::table)
        .values((posts::title.eq(title), posts::body.eq(body)))
        .execute(conn)
        .expect("Error saving new post");

    posts::table.order(posts::id.desc()).first(conn).unwrap()
}
