use actix_web::{web::Data, dev, HttpRequest, FromRequest,cookie::Cookie};
use futures_util::future::{ok, err, Ready};
use jwt_gang::claim_config::ClaimConfiguration;
use crate::err::XamXamWebError;
use actix_web::HttpMessage;

pub struct UserId(i32);

impl UserId {
    pub fn new(id : i32) -> Self { UserId(id) }

    pub fn get_id(&self) -> i32 { self.0 }
}

impl FromRequest for UserId {
    type Error = XamXamWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _ : &mut dev::Payload) -> Self::Future {
        let jwt_config : &ClaimConfiguration = match req.app_data::<Data<ClaimConfiguration>>() {
            Some(config) => config,
            None => return err(XamXamWebError::JwtConfigIsNotThere)
        };
        let cookie : Cookie = match req.cookie("Authorization") {
            Some(cookie) => cookie,
            None => return err(XamXamWebError::CredentialsNotPresent)
        };
        let split : Vec<&str> = cookie.value().split("Bearer").collect();
        let id_of_token = match jwt_config.decode_token(&split[1].trim()) {
            Ok(claim) => claim.claims.get_subject().to_owned(),
            Err(e) => return err(e.into())
        };
        let id : i32 = match id_of_token.parse::<i32>() {
            Ok(id) => id,
            Err(_) => return err(XamXamWebError::from("Could not parse string reference to i32"))
        };
        ok(UserId::new(id))
    }
}