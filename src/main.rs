use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(health_check)})
    .bind("127.0.0.1:8088")?
    .run()
    .await
}