use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to parse configuration");
    let listener =
        std::net::TcpListener::bind(format!("localhost:{}",config.http_port)).expect("test failed to find available port");
    return run(listener)?.await;
}
