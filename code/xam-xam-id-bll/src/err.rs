use std::{error::Error, fmt};
use serde::{Serialize,Deserialize};
use xam_xam_dal::err::XamXamError;

#[derive(Debug,Serialize,Deserialize)]
pub enum XamXamServiceError {
    //Dal error
    XamXamDalError(XamXamError),
    //User related error
    UserAlreadyInRedisDB,
    //Custom errors
    CustomError(String)
}

impl fmt::Display for XamXamServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            //Dal error
            XamXamServiceError::XamXamDalError(err) => write!(f,"{}",err),
            XamXamServiceError::UserAlreadyInRedisDB => write!(f,"A user can not be already present in the redis database"),
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

impl Error for XamXamServiceError { }