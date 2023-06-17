use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener =
        std::net::TcpListener::bind("localhost:0").expect("test failed to find available port");
    return run(listener)?.await;
}
