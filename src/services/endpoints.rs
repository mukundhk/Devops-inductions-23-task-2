
use actix_web::{Responder,web,Error,post,get};
use crate::services::db;
use crate::schema::users::user_name;
use crate::schema::users::dsl::users;
use crate::models::models::{User,CreateUser,RequestResponse,UserResponse,QueryUser};
use diesel::prelude::*;
use crate::DbPool;
use crate::services::db::create_user;


#[post("/createUser")]
pub async fn create_new_user(pool: web::Data<DbPool>,info : web::Json<CreateUser>) -> Result<impl Responder, Error>{
    println!("Check {}",info.user_name);
    
    let _new_user = web::block(move || {
        let conn = &mut pool.get().unwrap();
        create_user(conn,&info.user_name,&info.user_email,&info.user_password)
      });

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