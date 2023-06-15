use actix_web::{web, App, HttpResponse, HttpServer, Responder};

//Connecting to database
use login_app_backend::establish_connection;
use login_app_backend::create_user;
//For logging
use actix_web::middleware::Logger;
use env_logger::Env;



async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}

async fn create_new_user() -> impl Responder {
    let user_name = String::from("rithvik");
    let user_email = String::from("rithvik.ravilla@gmail.com");
    let user_password = String::from("password");
    let connection = &mut establish_connection();
    create_user(connection, &user_name,&user_email,&user_password);
    HttpResponse::Ok().body("I am adding user!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    println!("Connection to DB Established !\n");

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/create", web::get().to(create_new_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}