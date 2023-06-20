
use actix_web::{Responder,web,Error,post,get};
use crate::services::db;
use crate::schema::users::user_name;
use crate::schema::users::dsl::users;
use crate::models::models::{User,CreateUser,RequestResponse,UserResponse,QueryUser};
use diesel::prelude::*;


#[post("/createUser")]
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

#[get("/getUser")]
pub async fn get_user(info: web::Query<QueryUser>) -> Result<impl Responder, Error>{
    
    let connection = &mut db::establish_connection();
    let name = info.into_inner();

    let results = users
        .filter(user_name.eq(name.user_name))
        .select(User::as_select())
        .load(connection)
        .expect("Error loading posts");

    let final_message: UserResponse;

    if results.len() != 0 {
        final_message = UserResponse {
            user_name: results[0].clone().user_name,
            user_email: results[0].clone().user_email,
        };
    } else {
        final_message = UserResponse {
            user_name: String::from("Error"),
            user_email: String::from("Error"),
        };
    }
    Ok(web::Json(final_message))
}