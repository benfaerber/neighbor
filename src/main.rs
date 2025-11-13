use actix_web::{get, App, HttpResponse, HttpServer, Responder};

mod model;

#[get("/api")]
async fn api() -> impl Responder {
    HttpResponse::Ok().body("Hello from API!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(api)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
