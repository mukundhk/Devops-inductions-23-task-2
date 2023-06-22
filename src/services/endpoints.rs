use actix_web::{Responder,web,Error,post,get,put};
use crate::models::models::{CreateUser,RequestResponse,UpdateUser};
use crate::DbPool;
use crate::services::db::{create_user,get_all_user,get_some,update_user};

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

#[get("/getUser/{email}")]
pub async fn get_some_user(email: web:: Path <String>,pool: web::Data<DbPool>) -> Result<impl Responder, Error>{

      let details = email.into_inner();
      let found_users = web::block(move || {
        let conn = &mut pool.get().unwrap();
        get_some(conn,&details)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(web::Json(found_users))
}

#[put("/updateUser/{email}")]
pub async fn update_particular_user(email: web:: Path <String>,info : web::Json<UpdateUser>,pool: web::Data<DbPool>) -> Result<impl Responder, Error>{

  let details = email.into_inner();
      let found_users = web::block(move || {
        let conn = &mut pool.get().unwrap();
        update_user(conn,&details,&info.into_inner().user_password)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(web::Json(found_users))
}