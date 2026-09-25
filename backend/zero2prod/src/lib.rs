// crates
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;

// 3.7.3.1
#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}
// 3.7.3.1

// request handler
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// // 3.7
// We were returning `impl Responder` at the very beginning.
// We are now spelling out the type explicitly given that we have
// become more familiar with `actix_web`.
// There is no performance difference! Just a stylistic choice! :)
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
// // 3.7

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no `.await` call, so it is not needed anymore
// pub fn run() -> Result<Server, std::io::Error> {
//     let server = HttpServer::new(|| {
//         App::new()
//             .route("/health_check", web::get().to(health_check))
//         })
//         .bind("127.0.0.1:8000")?
//         .run();
//     // No .await here!
//     Ok(server)
// }

// 3.5.1: Polishing -> improve
// pub fn run(address: &str) -> Result<Server, std::io::Error> {
//     let server = HttpServer::new(|| {
//         App::new()
//             .route("/health_check", web::get().to(health_check))
//     })
//         .bind(address)?
//         .run();
//     // No .await here!
//     Ok(server)
// }

// ...
// Failed to execute request.: reqwest::Error { kind: Request, url: Url { scheme: "http",
// cannot_be_a_base: false, username: "", password: None, host: Some(Ipv4(127.0.0.1)),
// port: Some(8000), path: "/health_check", query: None, fragment: None },
// source: hyper::Error(Connect, ConnectError("tcp connect error", Os { code: 111,
// kind: ConnectionRefused, message: "Connection refused" })) }
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// the problem is currently our HTTP client is still calling `127.0.0.1:8000` and we really
// don't know what to put there now: the application port is determined at runtime, we can't
// hard code it there.
// We need, somehow, to find out what port the OS has gifted our application and return it from
// `spawn_app`.
// There are a few ways to go about it - we will use a `std::net::TcpListener`
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe)) // 3.7
    })
        .listen(listener)?
        .run();
    // No .await here!
    Ok(server)
}