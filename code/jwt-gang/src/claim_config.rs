use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Header, TokenData, Validation};
use serde::Deserialize;
use crate::claim::Claim;
use crate::claim_error::JwtCustomError;

/**
 * This configuration will be used to make claims and to validate these claims.
 * - claim_issuer : issuer is mostly the server name, used to identify 
 */
#[derive(Clone, Deserialize)]
pub struct ClaimConfiguration {
    claim_issuer : String,
    claim_secret : String,
    claim_expiration : usize
}

impl ClaimConfiguration {
    pub fn new(issuer : &str, secret : &str, expiration : usize) -> Self {
        Self {
            claim_issuer : issuer.to_string(),
            claim_secret : secret.to_string(),
            claim_expiration : expiration
        }
    }

    pub fn get_issuer(&self) -> &str {
        &self.claim_issuer
    }

    pub fn get_secret(&self) -> &[u8] {
        self.claim_secret.as_ref()
    }

    pub fn get_expiration(&self) -> usize {
        self.claim_expiration
    }

    pub fn create_claim(&self, user_id : &str) -> Result<Claim,JwtCustomError> {
        Ok(
            Claim::new(user_id, self.get_issuer(), self.get_expiration())?
        )
    }

    pub fn token_from_claim(&self, claim : &Claim) -> Result<String, JwtCustomError> {
        match encode(&Header::default(), &claim, self.get_secret()) {
            Ok(token) => {
                info!("A token has been made from a claim");
                Ok(token)
            }
            Err(e) => {
                warn!("A token couldn't be made out of a claim. Reason: {}", e);
                Err(JwtCustomError::TokenCannotBeMadeFromClaim)
            }
        }
    }

     /**
     * Function that decodes a token string returning a claim.
     *
     * An error can be thrown when:
     * * a token is empty
     * * Whenever the issuer of the decoded token is not equal to the issuer in the .env file
     * * token is invalid
     */
    pub fn decode_token(&self, token: &str) -> Result<TokenData<Claim>, JwtCustomError> {
        if token.is_empty() {
            warn!("JWT error: {}", JwtCustomError::TokenIsEmpty);
            return Err(JwtCustomError::TokenIsEmpty)
        }
        let mut validate: Validation = Validation::default();
        validate.iss = Some(self.get_issuer().to_string());
        match decode::<Claim>(
            &token,
            self.get_secret(),
            &validate,
        ) {
            Ok(c) => Ok(c),
            Err(err) => match &*err.kind() {
                ErrorKind::InvalidToken => {
                    warn!("JWT error: {}", JwtCustomError::TokenIsInvalid);
                    Err(JwtCustomError::TokenIsInvalid)
                }
                ErrorKind::InvalidIssuer => {
                    warn!("JWT error: {}", JwtCustomError::IssuerIsInvalid);
                    Err(JwtCustomError::IssuerIsInvalid)
                }
                ErrorKind::ExpiredSignature => {
                    warn!("JWT error: {}", JwtCustomError::SignatureHasExpired);
                    Err(JwtCustomError::SignatureHasExpired)
                }
                e => {
                    warn!("JWT error: {}",format!("Some other JWT error. Error: {:#?}",&e));
                    Err(JwtCustomError::CustomError(format!("Some other JWT error. Error: {:#?}",&e)))
                }
            },
        }
    }
}