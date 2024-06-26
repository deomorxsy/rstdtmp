mod configs;

use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use configs::cors::with_cors;
use actix_files::NamedFile;
use std::path::PathBuf;

#[get("/")]
async fn hello() -> impl Responder{
    HttpResponse::Ok().body("hello wrld")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
