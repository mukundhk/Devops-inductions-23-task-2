use actix_web::{Responder,web};
use crate::services::db;
use crate::HttpResponse;
use crate::schema::users::user_name;
use crate::schema::users::dsl::users;
use crate::models::models::User;
use diesel::prelude::*;

pub async fn create_new_user() -> impl Responder {
    let other_user_name = String::from("rithvik");
    let user_email = String::from("rithvik.ravilla@gmail.com");
    let user_password = String::from("password");
    let connection = &mut db::establish_connection();
    db::create_user(connection, &other_user_name,&user_email,&user_password);
    HttpResponse::Ok().body("I am adding user!")
}

pub async fn get_user(name: web::Path<String>) -> impl Responder {
    
    let connection = &mut db::establish_connection();
    let test_name = name.into_inner();

    let results = users
        .filter(user_name.eq(test_name))
        .select(User::as_select())
        .load(connection)
        .expect("Error loading posts");
    if results.len() == 0 {
        HttpResponse::Ok().body("Your man not here")
    } else {
        HttpResponse::Ok().body("Your guy is found")
    }
}