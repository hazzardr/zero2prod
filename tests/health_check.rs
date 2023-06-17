use std::net::TcpListener;

use reqwest::Client as HttpClient;

#[tokio::test]
async fn health_check_test() {
    // Given
    let address = spawn_app();

    let client = HttpClient::new();

    //when
    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to call localhost in integration test");

    //then
    assert!(&response.status().is_success());
}

// Localhost:0 will try to scan for an unused port
fn spawn_app() -> String {
    let listener = TcpListener::bind("localhost:0").expect("test failed to find available port");

    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("test is failing, couldnt start server");
    let _ = tokio::spawn(server);

    format!("http://localhost:{}", port)
}
