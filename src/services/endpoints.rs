use actix_web::{Responder,web,Error,post,get};
use crate::services::db;
use crate::schema::users::user_name;
use crate::schema::users::dsl::users;
use crate::models::models::{User,CreateUser,RequestResponse,UserResponse,QueryUser};
use diesel::prelude::*;
use crate::DbPool;
use crate::services::db::{create_user,get_all_user};

#[post("/createUser")]
pub async fn create_new_user(pool: web::Data<DbPool>,info : web::Json<CreateUser>) -> Result<impl Responder, Error>{

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
pub async fn get_all_present_user(pool: web::Data<DbPool>) -> Result<impl Responder, Error>{

      let found_users = web::block(move || {
        let conn = &mut pool.get().unwrap();
        get_all_user(conn)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(web::Json(found_users))
}

