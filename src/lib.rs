pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{NewSprint, sprint};


pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_sprint(conn: &mut MysqlConnection, sprint: &i8, sprint_date: &str) -> sprint {
    print!("hi\n");
    use schema::sprints::dsl::*;

    let new_sprint = NewSprint {sprint, sprint_data: "" };

    diesel::insert_into(sprints)
        .values(&new_sprint)
        .execute(conn)
        .expect("Error saving new sprint");

    sprints.order(sprint.desc()).first(conn).unwrap()
}