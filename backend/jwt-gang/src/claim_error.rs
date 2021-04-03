use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub enum JwtCustomError {
    EnvironmentalVariableMissing,
    TokenCannotBeMadeFromClaim,
    EmptySubjectOfToken,
    TokenIsEmpty,
    TokenIsInvalid,
    UserIdIsEmpty,
    IssuerIsInvalid,
    IssuerIsEmpty,
    SignatureHasExpired,
    ExpirationEqualsNull,
    CustomError(String),
}

impl fmt::Display for JwtCustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JwtCustomError::EnvironmentalVariableMissing => write!(
                f,
                "Could not get a environmental variable used to make a jwt configuration."
            ),
            JwtCustomError::TokenCannotBeMadeFromClaim => {
                write!(f, "Couldn't create a token out of a claim")
            }
            JwtCustomError::EmptySubjectOfToken => {
                write!(f, "A subject of a JWT token can't be empty")
            }
            JwtCustomError::TokenIsEmpty => write!(f, "A token string cannot be empty"),
            JwtCustomError::TokenIsInvalid => write!(f, "JWT token is invalid"),
            JwtCustomError::UserIdIsEmpty => write!(f, "The user id is empty"),
            JwtCustomError::IssuerIsInvalid => write!(f, "JWT token issuer is invalid"),
            JwtCustomError::IssuerIsEmpty => write!(f, "JWT token issuer is empty"),
            JwtCustomError::SignatureHasExpired => {
                write!(f, "Signature of jwt token has been expired")
            }
            JwtCustomError::ExpirationEqualsNull => {
                write!(f, "An expiration of jwt token cannot be equal or under 0")
            }
            JwtCustomError::CustomError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for JwtCustomError {}
