use actix_web::{get, web};
use mailgang::curl_mail::Mailer;
use crate::err::XamXamWebError;

#[get("/ping")]
pub async fn ping(mailer : web::Data<Mailer>) -> String {
    match mailer.send_mail("michael28072000@outlook.com", "Test mail", "Test", "<h1>Test</h1>") {
        Ok(_) => "Pong",
        Err(_) => "No Pong"
    }.to_string()
}

pub fn request_new_user()