//To actually use them
mod services { 
    //Adding pub makes it public and we can use the functions in the file
    pub mod db; 
    pub mod endpoints;
}
mod models {
    pub mod models;
}
//Using root level file
pub mod schema;

//By keeping all these files in main, [Intellisense/Rust sees them]
use actix_web::{web, error,App,middleware, HttpResponse, HttpServer, Responder};
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

        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });

        App::new()
            .wrap(middleware::Logger::default())
            .app_data(json_config)
            .route("/", web::get().to(index))
            .route("/create", web::post().to(create_new_user))
            .route("/get/{name}", web::get().to(get_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}