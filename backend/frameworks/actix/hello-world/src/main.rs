// Crates
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

// Request Handler
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

// Runtime
#[tokio::main]
async fn main() -> std::io::Result<()> { // future ready
    // Server
    HttpServer::new(|| {
        // Application
        App::new()
            // Route
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")? // Server address and TCP socket
    .run()
    .await
}
