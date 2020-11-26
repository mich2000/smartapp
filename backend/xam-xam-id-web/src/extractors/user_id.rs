use crate::err::XamXamWebError;
use actix_identity::RequestIdentity;
use actix_web::{dev, web::Data, FromRequest, HttpRequest};
use futures_util::future::{err, ok, Ready};
use jwt_gang::claim_config::ClaimConfiguration;

pub struct UserId(i32);

impl UserId {
    pub fn new(id: i32) -> Self {
        UserId(id)
    }

    pub fn get_id(&self) -> i32 {
        self.0
    }
}

impl FromRequest for UserId {
    type Error = XamXamWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let jwt_config: &ClaimConfiguration = match req.app_data::<Data<ClaimConfiguration>>() {
            Some(config) => config,
            None => return err(XamXamWebError::JwtConfigIsNotThere),
        };
        let identity = match req.get_identity() {
            Some(cookie) => cookie,
            None => return err(XamXamWebError::CredentialsNotPresent),
        };
        let id_of_token = match jwt_config.decode_token(&identity) {
            Ok(claim) => claim.claims.get_subject().to_owned(),
            Err(e) => return err(e.into()),
        };
        let id: i32 = match id_of_token.parse::<i32>() {
            Ok(id) => id,
            Err(_) => {
                return err(XamXamWebError::from(
                    "Could not parse string reference to i32",
                ))
            }
        };
        ok(UserId::new(id))
    }
}
