use std::{error::Error, fmt};
use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize,Clone)]
pub enum JwtCustomError {
    TokenCannotBeMadeFromClaim,
    EmptySubjectOfToken,
    TokenIsEmpty,
    TokenIsInvalid,
    IssuerIsInvalid,
    SignatureHasExpired,
    ExpirationEqualsNull,
    CustomError(String)
}

impl fmt::Display for JwtCustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JwtCustomError::TokenCannotBeMadeFromClaim => write!(f,"Couldn't create a token out of a claim"),
            JwtCustomError::EmptySubjectOfToken => write!(f,"A subject of a JWT token can't be empty"),
            JwtCustomError::TokenIsEmpty => write!(f,"A token string cannot be empty"),
            JwtCustomError::TokenIsInvalid => write!(f,"JWT token is invalid"),
            JwtCustomError::IssuerIsInvalid => write!(f,"JWT token issuer is invalid"),
            JwtCustomError::SignatureHasExpired => write!(f,"Signature of jwt token has been expired"),
            JwtCustomError::ExpirationEqualsNull => write!(f,"An expiration of jwt token cannot be equal or under 0"),
            JwtCustomError::CustomError(e) => write!(f,"{}",e)
        }
    }
}

impl Error for JwtCustomError { }