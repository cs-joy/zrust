// crate
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// request handler
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

// runtime
#[tokio::main]
async fn main() -> std::io::Result<()> {
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
///////////////////////////////////////////////////////////
// test manually via `curl` and expected output
// $ curl -v http://127.0.0.1:8000/health_check
/////// Output ////////////////////////////////
// *   Trying 127.0.0.1:8000...
// * Connected to 127.0.0.1 (127.0.0.1) port 8000
// > GET /health_check HTTP/1.1
// > Host: 127.0.0.1:8000
// > User-Agent: curl/8.5.0
// > Accept: */*
// >
// < HTTP/1.1 200 OK
// < content-length: 0
// < date: Thu, 17 Sep 2026 03:15:32 GMT
// <
// * Connection #0 to host 127.0.0.1 left intact
///////////////////////////////////////////////////////////