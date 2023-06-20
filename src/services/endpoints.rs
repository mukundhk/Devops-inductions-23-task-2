
use actix_web::{Responder,web,Error,post};
use crate::services::db;
use crate::HttpResponse;
use crate::schema::users::user_name;
use crate::schema::users::dsl::users;
use crate::models::models::{User,CreateUser,RequestResponse};
use diesel::prelude::*;


#[post("/create")]
pub async fn create_new_user(info : web::Json<CreateUser>) -> Result<impl Responder, Error>{
    println!("Check {}",info.user_name);
    let connection = &mut db::establish_connection();
    // println!("Check 2");
    db::create_user(connection, &info.user_name,&info.user_email,&info.user_password);
    // println!("Check 3");

    let message = RequestResponse {
        message:"User has been created".to_string(),
    };
    Ok(web::Json(message))
}

pub async fn get_user(path: web::Path<String>) -> impl Responder {
    
    let connection = &mut db::establish_connection();
    let name = path.into_inner();

    let results = users
        .filter(user_name.eq(name))
        .select(User::as_select())
        .load(connection)
        .expect("Error loading posts");
    if results.len() == 0 {
        HttpResponse::Ok().body("Your man not here")
    } else {
        HttpResponse::Ok().body("Your guy is found")
    }
}