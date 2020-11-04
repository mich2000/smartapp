use actix_web::{dev, HttpRequest, FromRequest};
use futures_util::future::{ok, err, Ready};
use crate::err::XamXamWebError;
use actix_web_httpauth::headers::authorization::{Authorization,Basic};
use actix_web::http::header::Header;

pub struct Cred(String,String);

impl Cred {
    pub fn new(username : &str, password : &str) -> Self { Cred(username.to_owned(),password.to_owned()) }

    pub fn get_name(&self) -> &str { &self.0 }
    
    pub fn get_password(&self) -> &str { &self.1 }
}

impl FromRequest for Cred {
    type Error = XamXamWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _ : &mut dev::Payload) -> Self::Future {
        let credentials = match Authorization::<Basic>::parse(req) {
            Ok(credentials_scheme) => credentials_scheme,
            Err(_) => return err(XamXamWebError::CredentialsNotPresent)
        };
        let password : String = match credentials.as_ref().password() {
            Some(pwd) => pwd.to_owned().to_string(),
            None => return err(XamXamWebError::CredentialsNotPresent)
        };
        let username : String = credentials.as_ref().user_id().to_string();
        ok(Cred::new(&username, &password))
    }
}