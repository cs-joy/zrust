use std::net::TcpListener;
use robust_integration_test::run_server;

#[tokio::test]
async fn health_check_test() {
    // Arrange
    let address = spawn_app();

    // prepare HTTP client
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("http://{}/health", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn hello_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://{}/hello", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(12), response.content_length());
}

fn spawn_app() -> String {
    // bind port with TcpListener
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // port -> retrieving the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    println!("listening port: {}", port);

    let server = run_server(listener).expect("Failed to run server");
    let _ = tokio::spawn(server);

    // return application address
    format!("127.0.0.1:{}", port)
}