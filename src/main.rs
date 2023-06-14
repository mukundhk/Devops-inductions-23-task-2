use actix_web::{web, App, middleware, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    
    
    
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}