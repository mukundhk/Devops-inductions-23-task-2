use actix_web::Responder;
use crate::services::db;
use crate::HttpResponse;
use crate::schema::users::dsl::users;
use crate::models::models::User;
use diesel::prelude::*;

pub async fn create_new_user() -> impl Responder {
    let user_name = String::from("rithvik");
    let user_email = String::from("rithvik.ravilla@gmail.com");
    let user_password = String::from("password");
    let connection = &mut db::establish_connection();
    db::create_user(connection, &user_name,&user_email,&user_password);
    HttpResponse::Ok().body("I am adding user!")
}

pub async fn get_user() -> impl Responder {
    let connection = &mut db::establish_connection();
    let results = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading posts");
    let name = &results[0];
    let actual_name = &name.user_name;
    println!("I hate this language so much {}",actual_name);
    HttpResponse::Ok().body("This bs is working")
}