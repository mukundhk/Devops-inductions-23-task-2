use actix_web::{web, App, HttpResponse, HttpServer, Responder};

//Connecting to database
use login_app_backend::establish_connection;
//For logging
use actix_web::middleware::Logger;
use env_logger::Env;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    
    let _connection = &mut establish_connection();
    println!("Connection to DB Established \n");

    // env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    // env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}