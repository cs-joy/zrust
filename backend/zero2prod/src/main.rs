use actix_web::{web, App, HttpRequest, HttpServer, Responder};

// necessary function
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main] // Runtime
async fn main() -> std::io::Result<()> {
    // Server
    HttpServer::new(|| {
        // Application
        App::new()
            // Endpoint
            .route("/", web::get().to(greet)) // GET / endpoint
            .route("/{name}", web::get().to(greet)) // GET /{name} endpoint
    })
    .bind("127.0.0.1:8000")?// TCP socket
    .run()
    .await
}