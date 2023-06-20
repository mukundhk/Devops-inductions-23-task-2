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

//For logging
use env_logger::Env;
use crate::services::endpoints::create_new_user;
use crate::services::endpoints::get_user;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}


//Making a conneciton pool
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {    


    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let database_url = String::from("postgres://postgres:Try2read@localhost:5432/rust_server");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    println!("Connection to DB Established !\n");

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {

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
            .app_data(web::Data::new(pool.clone()))
            .service(create_new_user)
            .service(get_user)
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}