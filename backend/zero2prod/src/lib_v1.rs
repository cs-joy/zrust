// crate
use actix_web::{web, App, HttpResponse, HttpServer};

// request handler
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// We need to mark `run` as public.
// It is also no longer a binary entrypoint, therefore we can mark it as async
// without having to use any pro-macro incantation
pub async fn run() -> std::io::Result<()> /* future trait */ {
    // server
    HttpServer::new(|| {
        // application
        App::new()
            // route
            .route("/health_check", web::get().to(health_check))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}