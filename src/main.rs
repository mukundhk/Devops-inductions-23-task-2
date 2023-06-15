use actix_web::{web, App,middleware, HttpResponse, HttpServer, Responder};
//Connecting to database
use login_app_backend::models::User;
use login_app_backend::schema::users::dsl::users;
//For logging
use env_logger::Env;
use diesel::prelude::*;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}

async fn create_new_user() -> impl Responder {
    let user_name = String::from("rithvik");
    let user_email = String::from("rithvik.ravilla@gmail.com");
    let user_password = String::from("password");
    let connection = &mut login_app_backend::establish_connection();
    login_app_backend::create_user(connection, &user_name,&user_email,&user_password);
    HttpResponse::Ok().body("I am adding user!")
}

async fn get_user() -> impl Responder {
    let connection = &mut login_app_backend::establish_connection();
    let results = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading posts");
    let name = &results[0];
    let actual_name = &name.user_name;
    println!("I hate this language so much {}",actual_name);
    HttpResponse::Ok().body("This bs is working")
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