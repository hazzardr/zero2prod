use actix_web::web::Form;
use actix_web::HttpResponse;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub user: String,
    pub email: String,
}

pub fn index(form: Form<FormData>) -> String {
    format!("welcome! {}", form.user)
}

pub async fn subscribe(_form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}