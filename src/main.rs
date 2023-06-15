//To actually use them
mod services { 
    pub mod db; 
    pub mod endpoints;}
mod models {pub mod models;}
pub mod schema;

use actix_web::{web, App,middleware, HttpResponse, HttpServer, Responder};
//Connecting to database

//For logging
use env_logger::Env;
use crate::services::endpoints::create_new_user;
use crate::services::endpoints::get_user;


async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    println!("Connection to DB Established !\n");

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/create", web::get().to(create_new_user))
            .route("/get", web::get().to(get_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}