use chrono::prelude::*;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use crate::jwt_numeric_date;

/**
 * Claim is used to prove authorization for an user for a certain amount of time.
 *
 * Attributes:
 * * sub : is an id of an user
 * * iss : is the issuer of the claim
 * * exp : datetime which indicates the date that it will be valid
 * * iat : datetime the claim was issued
 * * is_admin : Claim that is used to identify if the user is an administrator
 */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Claim {
    pub sub: String,
    pub iss: String,
    #[serde(with = "jwt_numeric_date")]
    pub exp: DateTime<Utc>,
    #[serde(with = "jwt_numeric_date")]
    pub iat: DateTime<Utc>
}

impl Claim {
    pub fn new(
        subject : &str,
        issuer : &str,
        expiration : i64
    ) -> Result<Claim, &'static str> {
        if subject.is_empty() {
            warn!("The subject of the jwt claim is empty");
            return Err("A subject of a JWT token can't be empty")
        }
        let today = Utc::now();
        Ok(
            Self {
                sub: subject.to_string(),
                iss: issuer.to_string(),
                exp: today + chrono::Duration::seconds(expiration),
                iat: today
            }
        )
    }
}