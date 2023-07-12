use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web::{self, Form},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};

#[derive(serde::Deserialize)]
struct FormData {
    user: String,
    email: String,
}

fn index(form: web::Form<FormData>) -> String {
    format!("welcome! {}", form.user)
}

async fn subscribe(_form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

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
