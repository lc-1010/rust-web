use actix_web::{get, web, App, HttpServer,HttpResponse, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/")]
async fn hello()->impl Responder {
    HttpResponse::Ok().body("hello world!")
}

async fn manual_hello() ->impl Responder{
    HttpResponse::Ok().body("hey there!")
}
#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(hello)
            .route("/hey",web::get().to(manual_hello))

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}