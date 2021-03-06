use crate::claim_error::JwtCustomError;
use crate::jwt_numeric_date;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

/**
 * Claim is used to prove authorization for an user for a certain amount of time.
 *
 * Attributes:
 * * sub : is an id of an user
 * * iss : is the issuer of the claim
 * * exp : datetime which indicates the date that it will be valid
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Claim {
    iss: String,
    sub: String,
    #[serde(with = "jwt_numeric_date")]
    exp: DateTime<Utc>,
}

impl Claim {
    pub fn new(subject: &str, issuer: &str, expiration: u64) -> Result<Claim, JwtCustomError> {
        if expiration == 0 {
            warn!("JWT claim cannot have a expiration equal to 0.");
            return Err(JwtCustomError::ExpirationEqualsNull);
        }
        if subject.is_empty() {
            warn!("The subject of the jwt claim is empty");
            return Err(JwtCustomError::EmptySubjectOfToken);
        }
        Ok(Self {
            sub: subject.to_string(),
            iss: issuer.to_string(),
            exp: Utc::now() + chrono::Duration::seconds(expiration as i64),
        })
    }

    pub fn get_subject(&self) -> &str {
        &self.sub
    }

    pub fn get_issuer(&self) -> &str {
        &self.iss
    }
}
