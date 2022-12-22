use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::io;

// config route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Actix Web Service is running!")
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("hello world!")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let app = move || App::new().configure(general_routes).service(home);
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
