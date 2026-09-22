use std::net::TcpListener;
use robust_integration_test::run_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    println!("Listening on: {}", listener.local_addr()?);
    run_server(listener)?.await
}