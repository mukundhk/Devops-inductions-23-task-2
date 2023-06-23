use diesel::{pg::PgConnection};
use diesel::result::Error;
use diesel::prelude::*;
// use dotenvy::dotenv;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
// use std::env;
use crate::models::models::{NewUser,User};

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let database_url = String::from("postgres://postgres:Try2read@localhost:5432/rust_server");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    return pool;
}

pub fn create_user(conn: &mut PgConnection, user_name: &str, user_email: &str, user_password: &str) -> User {
    use crate::schema::users;

    let new_user = NewUser { user_name, user_email,user_password };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn get_all_user(conn: &mut PgConnection) -> Result<Vec<User>,Error> {
    use crate::schema::users::dsl::*;

    let items = users.load::<User>(conn);
    return items;
}

pub fn get_some(conn: &mut PgConnection,email: &str) -> Result<User,Error> {
    use crate::schema::users::dsl::*;

    let items = users.filter(user_email.eq(email)).first::<User>(conn);
    return items;
}

pub fn update_user(conn: &mut PgConnection,email: &str,password: &str) -> Result<User,Error> {
    use crate::schema::users::dsl::*;

    let person = get_some(conn,email).unwrap();
    let item = diesel::update(users.find(person.id))
    .set(user_password.eq(password))
    .get_result::<User>(conn)?;
    Ok(item)
}

pub fn delete_user(conn: &mut PgConnection,email: &str) -> Result<usize,Error> {
    use crate::schema::users::dsl::*;

    let person = get_some(conn,email).unwrap();
    let count = diesel::delete(users.find(person.id)).execute(conn)?;
    Ok(count)
}