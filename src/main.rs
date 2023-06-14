use actix_web::{web, App, middleware, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}

async fn user_details(user_id:  web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("User has ID: {}",user_id))

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/user/{user_id}",web::get().to(user_details))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}