use actix_web::{ dev, web::Data, HttpRequest, FromRequest};
use futures_util::future::{ok, err, Ready};
use jwt_gang::claim_config::ClaimConfiguration;
use crate::err::XamXamWebError;

#[derive(Debug)]
pub struct UserId(String);

impl UserId {
    pub fn user_id(&self) -> &str {
        &self.0
    }
}

/**
 * Extractor that serves to extract the subject from the JWT token from the user to get the user id of that request to then later use.
*/
impl FromRequest for UserId {
    type Error = XamXamWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _ : &mut dev::Payload) -> Self::Future {
        let jwt_config : &Data<ClaimConfiguration> = match req.app_data::<Data<ClaimConfiguration>>() {
            Some(some_jwt_config) => some_jwt_config,
            None => return err(XamXamWebError::JwtConfigIsNotThere)
        };
        let auth = match req.headers().get("Authorization") {
            Some(auth) => match auth.to_str() {
                Ok(auth_string) => auth_string,
                Err(_) => return err(XamXamWebError::from("Could not parse a header to a string"))
            },
            None => return err(XamXamWebError::CredentialsNotPresent)
        };
        let split : Vec<&str> = auth.split("Bearer").collect();
        let token = split[1].trim();
        match jwt_config.as_ref().decode_token(token) {
            Ok(jwt_token) => ok(UserId(jwt_token.claims.get_subject().to_string())),
            Err(e) => err(e.into())
        }
    }
}