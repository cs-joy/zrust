// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name f the test file)
# [tokio::test]
async fn health_check_works() {
    // Arrange
    // spawn_app().await.expect("Failed to spawn our app");
    spawn_app(); // no .await, no .expect

    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background ~somehow~
///////////////////////////////////////////////////// v1
// async fn spawn_app() -> std::io::Result<()> {
//     lib_v1::run().await
// }

// No .await call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it is not worth it to propagate errors:
// if we fail to perform the required setup we can just panic and
// crash all the things
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to find address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have to use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
}