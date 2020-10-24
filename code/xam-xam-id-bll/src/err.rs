use std::{error::Error, fmt};
use serde::{Serialize,Deserialize};
use xam_xam_dal::err::XamXamError;
use jwt_gang::claim_error::JwtCustomError;

#[derive(Debug,Serialize,Deserialize)]
pub enum XamXamServiceError {
    //Dal error
    XamXamDalError(XamXamError),
    //User related error
    UserAlreadyInRedisDB,
    TokenNotCorrectForUserCreation,
    TokenNotCorrectForForgottenPwd,
    TokenNotCorrectForChangingEmail,
    // JWT errors
    JWTerror(JwtCustomError),
    //Custom errors
    CustomError(String)
}

impl fmt::Display for XamXamServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            //Dal error
            XamXamServiceError::XamXamDalError(err) => write!(f,"{}",err),
            //User related error
            XamXamServiceError::UserAlreadyInRedisDB => write!(f,"A user can not be already present in the redis database"),
            XamXamServiceError::TokenNotCorrectForUserCreation => write!(f,"Token that was given is not right, to create a new user"),
            XamXamServiceError::TokenNotCorrectForForgottenPwd => write!(f,"Token that was given is not right, to change the forgotten password"),
            XamXamServiceError::TokenNotCorrectForChangingEmail => write!(f,"Token that was given is not right, to change the email"),
            // JWT errors
            XamXamServiceError::JWTerror(err) => write!(f,"{}",err),
            // Custom errors
            XamXamServiceError::CustomError(e) => write!(f,"{}",e)
        }
    }
}

impl From<XamXamError> for XamXamServiceError {
    fn from(err : XamXamError) -> Self { XamXamServiceError::XamXamDalError(err) }
}

impl From<redis::RedisError> for XamXamServiceError { 
    fn from (err : redis::RedisError) -> Self { XamXamServiceError::CustomError(err.to_string()) } 
}

impl From<&str> for XamXamServiceError { 
    fn from (err : &str) -> Self { XamXamServiceError::CustomError(err.to_string()) } 
}

impl From<JwtCustomError> for XamXamServiceError {
    fn from(err : JwtCustomError) -> Self { XamXamServiceError::JWTerror(err) }
}

impl Error for XamXamServiceError { }

impl xam_xam_common::err_trait::PublicErrorTrait for XamXamServiceError {
    fn show_public_error(&self) -> String {
        if let XamXamServiceError::XamXamDalError(value) = &self {
            return value.show_public_error()
        }
        if let XamXamServiceError::JWTerror(value) = &self {
            return value.show_public_error()
        }
        match self {
            //User related error
            XamXamServiceError::TokenNotCorrectForUserCreation => "Token that was given is not correct, to create a new user",
            XamXamServiceError::TokenNotCorrectForForgottenPwd => "Token that was given is not right, to change the forgotten password",
            XamXamServiceError::TokenNotCorrectForChangingEmail => "Token that was given is not right, to change the email",
            _ => "An internal error happened"
        }.to_string()
    }
}