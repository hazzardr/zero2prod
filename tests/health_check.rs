use reqwest::Client as HttpClient;
use std::net::TcpListener;

#[tokio::test]
async fn member_can_subscribe_test() {
    // Given
    let address = spawn_app();
    let client = HttpClient::new();
    let body = "user=le%20guin&email=ursula_le_guin%40gmail.com";

    // When
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute request in test");

    // Then
    assert_eq!(200, response.status().as_u16())
}

#[tokio::test]
async fn missing_data_test() {
    // Given
    let address = spawn_app();
    let client = HttpClient::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    // When
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute request in test");

        // Then
        assert_eq!(
            400,
            response.status().as_u16(),
            "Expected status 400 with input payload of {}",
            error_message
        );
    }
}

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
