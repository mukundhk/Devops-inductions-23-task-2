use diesel::{pg::PgConnection};
use diesel::result::Error;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use crate::models::models::{NewUser,User, CreateUser, UpdateUser};
// use std::env;
// use dotenvy::dotenv;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let database_url = String::from("postgres://postgres:Try2read@localhost:5432/rust_server");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    return pool;
}

pub fn create_user(conn: &mut PgConnection, info: &mut CreateUser) -> Result<User,Error> {
    use crate::schema::users;
    let new_user = NewUser { 
        user_name: &info.user_name
        , user_email: &info.user_email
        ,user_password: &info.user_password 
    };
    let create = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)?;
    Ok(create)
}

pub fn get_all_users(conn: &mut PgConnection) -> Result<Vec<User>,Error> {
    use crate::schema::users::dsl::*;
    let items = users.load::<User>(conn)?;
    Ok(items)
}

pub fn get_users(conn: &mut PgConnection,email: &str) -> Result<User,Error> {
    use crate::schema::users::dsl::*;
    let items = users.filter(user_email.eq(email)).first::<User>(conn)?;
    Ok(items)
}

pub fn update_user(conn: &mut PgConnection,email: &str,update_details: &UpdateUser) -> Result<User,Error> {
    use crate::schema::users::dsl::*;
    let person = get_users(conn,email).unwrap();
  
  let mut final_password = person.user_password;
  match &update_details.user_password {
    Some(x) => final_password = x.to_string(),
    None => {},
  }

  let mut final_name = person.user_name;
  match &update_details.user_name {
    Some(x) => final_name = x.to_string(),
    None => {},
  }

    let item = diesel::update(users.filter(id.eq(person.id)))
    .set((user_name.eq(final_name),user_password.eq(final_password)))
    .get_result::<User>(conn)?;
    Ok(item)
}

pub fn delete_user(conn: &mut PgConnection,email: &str) -> Result<usize,Error> {
    use crate::schema::users::dsl::*;
    let person = get_users(conn,email).unwrap();
    let count = diesel::delete(users.find(person.id)).execute(conn)?;
    Ok(count)
}