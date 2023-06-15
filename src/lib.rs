pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //Future plans: Ok prints out connection established
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))    
}


use self::models::{NewUser, User};

pub fn create_post(conn: &mut PgConnection, user_name: &str, user_email: &str, user_password: &str) -> User {
    use crate::schema::users;

    let new_user = NewUser { user_name, user_email,user_password };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}