use std::{error::Error, fmt};
use serde::{Serialize,Deserialize};
use xam_xam_dal::err::XamXamError;

#[derive(Debug,Serialize,Deserialize)]
pub enum XamXamServiceError {
    //Dal error
    XamXamDalError(XamXamError),
    //Custom errors
    CustomError(String)
}

impl fmt::Display for XamXamServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            //Dal error
            XamXamServiceError::XamXamDalError(err) => write!(f,"{}",err),
            // Custom errors
            XamXamServiceError::CustomError(e) => write!(f,"{}",e)
        }
    }
}

impl From<XamXamError> for XamXamServiceError {
    fn from(err : XamXamError) -> Self { XamXamServiceError::XamXamDalError(err) }
}

impl Error for XamXamServiceError { }