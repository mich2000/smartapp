use std::{error::Error, fmt};
use xam_xam_dal::err::XamXamError;

#[derive(Debug, Clone)]
pub enum XamXamServiceError {
    //Dal error
    XamXamDalError(XamXamError),
    //Storage specific
    StorageNameIsEmpty,
    //Custom errors
    CustomError(String),
}

impl fmt::Display for XamXamServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            //Dal error
            XamXamServiceError::XamXamDalError(err) => write!(f, "{}", err),
            XamXamServiceError::StorageNameIsEmpty => write!(f, "A storage name cannot be empty."),
            // Custom errors
            XamXamServiceError::CustomError(e) => write!(f, "{}", e),
        }
    }
}

impl From<XamXamError> for XamXamServiceError {
    fn from(err: XamXamError) -> Self {
        error!("{}", err);
        XamXamServiceError::XamXamDalError(err)
    }
}

impl From<&str> for XamXamServiceError {
    fn from(err: &str) -> Self {
        error!("{}", err);
        XamXamServiceError::CustomError(err.to_string())
    }
}

impl Error for XamXamServiceError {}
