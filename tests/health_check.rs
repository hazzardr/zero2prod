use reqwest::Client as HttpClient;
use std::net::TcpListener;
use sqlx::{PgConnection, Connection};
use zero2prod::configuration::get_configuration;

#[tokio::test]
async fn member_can_subscribe_test() {
    // Given
    let address = spawn_app();
    let config = get_configuration().expect("Failed to read config");
    let db_connect_string = config.database.connection_string();
    let mut connection = PgConnection::connect(&db_connect_string)
        .await
        .expect("Failed to connect to postgres");
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
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("select email, name from subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("failed to fetch saved subscription");


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
    let server = zero2prod::startup::run(listener).expect("test is failing, couldnt start server");
    let _ = tokio::spawn(server);

    format!("http://localhost:{}", port)
}
