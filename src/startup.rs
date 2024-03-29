use std::net::TcpListener;
use actix_web::{
    dev::Server,
    web::{self},
    App, HttpServer
};
use routes::health_check;
use routes::subscribe;
use crate::routes;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
        .listen(listener)?
        .run();

    return Ok(server);
}
