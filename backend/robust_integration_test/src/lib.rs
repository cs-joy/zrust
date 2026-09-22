// crates
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;

// request handler
async fn api_health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Rust!\n")
}

pub fn run_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(api_health))
            .route("/hello", web::get().to(hello))
    })
    .listen(listener)?
    .run();
    Ok(server)
}