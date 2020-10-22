use actix_web::{get, web, App, HttpServer};
use mailgang::mailer_gang::Mailer;

#[get("/ping")]
pub async fn ping(mailer : web::Data<Mailer>) -> String {
    match mailer.test_mail_to_self() {
        Ok(_) => "Pong",
        Err(_) => "No Pong"
    }.to_string()
}