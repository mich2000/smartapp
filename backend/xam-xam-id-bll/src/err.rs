use std::{error::Error, fmt};
use xam_xam_dal::err::XamXamError;
use jwt_gang::claim_error::JwtCustomError;

#[derive(Debug,Clone)]
pub enum XamXamServiceError {
    //Dal error
    XamXamDalError(XamXamError),
    //User related error
    UserAlreadyInRedisDB,
    TokenNotCorrectForUserCreation,
    TokenNotCorrectForForgottenPwd,
    TokenNotCorrectForChangingEmail,
    TokenHasNotCorrectLength,
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
            XamXamServiceError::TokenHasNotCorrectLength => write!(f,"Given token has not the correct lenght."),
            // JWT errors
            XamXamServiceError::JWTerror(err) => write!(f,"{}",err),
            // Custom errors
            XamXamServiceError::CustomError(e) => write!(f,"{}",e)
        }
    }
}

impl From<XamXamError> for XamXamServiceError {
    fn from(err : XamXamError) -> Self { 
        error!("{}",err);
        XamXamServiceError::XamXamDalError(err)
    }
}

impl From<r2d2_redis::redis::RedisError> for XamXamServiceError {
    fn from (err : r2d2_redis::redis::RedisError) -> Self {
        error!("{}",err);
        XamXamServiceError::CustomError(format!("{}",err))
    }
}

impl From<&str> for XamXamServiceError { 
    fn from (err : &str) -> Self {
        error!("{}",err);
        XamXamServiceError::CustomError(err.to_string())
    } 
}

impl From<JwtCustomError> for XamXamServiceError {
    fn from(err : JwtCustomError) -> Self { 
        error!("{}",err);
        XamXamServiceError::JWTerror(err)
    }
}

impl Error for XamXamServiceError { }