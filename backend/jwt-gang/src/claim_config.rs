use crate::claim::Claim;
use crate::claim_error::JwtCustomError;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};

/**
 * This configuration will be used to make claims and to validate these claims. The maximum expiration that can be given is 2^16 expiration.
 */
#[derive(Clone)]
pub struct ClaimConfiguration<'a> {
    claim_decoder: DecodingKey<'a>,
    claim_encoder: EncodingKey,
    validation: Validation,
}

impl<'a> ClaimConfiguration<'a> {
    pub fn new(issuer: &str, secret: &'a str, expiration: u64) -> Result<Self, JwtCustomError> {
        if issuer.is_empty() {
            return Err(JwtCustomError::IssuerIsEmpty);
        }
        if secret.is_empty() {
            return Err(JwtCustomError::SecretIsEmpty);
        }
        if expiration >= i64::MAX as u64 {
            return Err(JwtCustomError::ExpirationIsTooBig);
        }
        Ok(Self {
            claim_decoder: DecodingKey::from_secret(secret.as_ref()),
            claim_encoder: EncodingKey::from_secret(secret.as_ref()),
            validation: Validation {
                iss: Some(issuer.to_string()),
                leeway: expiration,
                ..Default::default()
            },
        })
    }

    pub fn get_expiration(&self) -> u64 {
        self.validation.leeway
    }

    pub fn create_claim(&self, user_id: &str) -> Result<Claim, JwtCustomError> {
        if user_id.is_empty() {
            return Err(JwtCustomError::UserIdIsEmpty);
        }
        let issuer = match &self.validation.iss {
            Some(iss) => iss,
            None => return Err(JwtCustomError::IssuerIsEmpty),
        };
        Claim::new(user_id, issuer, self.get_expiration())
    }

    pub fn token_from_claim(&self, claim: &Claim) -> Result<String, JwtCustomError> {
        match encode(&Header::default(), &claim, &self.claim_encoder) {
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
            warn!("{}", JwtCustomError::TokenIsEmpty);
            return Err(JwtCustomError::TokenIsEmpty);
        }
        match decode::<Claim>(&token, &self.claim_decoder, &self.validation) {
            Ok(c) => Ok(c),
            Err(err) => match &*err.kind() {
                ErrorKind::InvalidToken => {
                    warn!("{}", JwtCustomError::TokenIsInvalid);
                    Err(JwtCustomError::TokenIsInvalid)
                }
                ErrorKind::InvalidIssuer => {
                    warn!("{}", JwtCustomError::IssuerIsInvalid);
                    Err(JwtCustomError::IssuerIsInvalid)
                }
                ErrorKind::ExpiredSignature => {
                    warn!("{}", JwtCustomError::SignatureHasExpired);
                    Err(JwtCustomError::SignatureHasExpired)
                }
                e => {
                    warn!("{}", format!("Some other JWT error. Error: {:#?}", &e));
                    Err(JwtCustomError::CustomError(format!(
                        "Some other JWT error. Error: {:#?}",
                        &e
                    )))
                }
            },
        }
    }
}

#[test]
fn test_sizes() {
    pub fn show_size<T>() {
        println!("{}", std::mem::size_of::<T>());
    }
    show_size::<Validation>();
    show_size::<DecodingKey>();
    show_size::<EncodingKey>();
    show_size::<ClaimConfiguration>();
    show_size::<usize>();
    show_size::<u16>();
}
