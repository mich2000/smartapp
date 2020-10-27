use std::{error::Error, fmt};
use serde::{Serialize,Deserialize};
use xam_xam_id_bll::err::XamXamServiceError;
use actix_web::{dev::HttpResponseBuilder, error, http::header, http::StatusCode, HttpResponse};

#[derive(Debug,Serialize,Deserialize)]
pub enum XamXamWebError {
    //Service related error
    ServiceError(XamXamServiceError),
    //DB related error
    CouldNotGetRedisConnection,
    CouldNotGetPostGresConnection,
    //Custom errors
    CustomError(String)
}

impl fmt::Display for XamXamWebError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            //Service
            XamXamWebError::ServiceError(err) => write!(f,"{}",err),
            //DB related error
            XamXamWebError::CouldNotGetRedisConnection => write!(f,"Could not get the redis connection from the redis pool"),
            XamXamWebError::CouldNotGetPostGresConnection => write!(f,"Could not get the postgres connection from the postgres pool"),
            // Custom errors
            XamXamWebError::CustomError(e) => write!(f,"{}",e)
        }
    }
}

impl From<&str> for XamXamWebError { 
    fn from (err : &str) -> Self { XamXamWebError::CustomError(err.to_string()) } 
}

impl From<xam_xam_id_bll::err::XamXamServiceError> for XamXamWebError {
    fn from(err : xam_xam_id_bll::err::XamXamServiceError) -> Self {
        XamXamWebError::ServiceError(err)
    }
}

impl From<jwt_gang::claim_error::JwtCustomError> for XamXamWebError {
    fn from(err : jwt_gang::claim_error::JwtCustomError) -> Self {
        XamXamWebError::ServiceError(XamXamServiceError::JWTerror(err))
    }
}

impl Error for XamXamWebError { }

impl xam_xam_common::err_trait::PublicErrorTrait for XamXamWebError {
    fn show_public_error(&self) -> String {
        error!("{}", self.show_public_error());
        if let XamXamWebError::ServiceError(value) = &self {
            return value.show_public_error()
        }
        "An internal error happened".to_string()
    }
}

#[derive(Debug,Deserialize, Serialize)]
pub struct XamActixError {
    pub error : String
}

impl XamActixError {
    pub fn new<E : std::error::Error + xam_xam_common::err_trait::PublicErrorTrait>(error : &E) -> Self {
        error!("{}",error);
        Self {
            error : error.show_public_error()
        }
    }
}

use xam_xam_common::err_trait::PublicErrorTrait;

impl error::ResponseError for XamXamWebError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
        .finish()
    }

    fn status_code(&self) -> StatusCode { 
        match self {
            XamXamWebError::ServiceError(service_err) => match service_err {
                //User related error
                XamXamServiceError::UserAlreadyInRedisDB => StatusCode::UNAUTHORIZED,
                XamXamServiceError::TokenNotCorrectForUserCreation => StatusCode::UNAUTHORIZED,
                XamXamServiceError::TokenNotCorrectForForgottenPwd => StatusCode::UNAUTHORIZED,
                XamXamServiceError::TokenNotCorrectForChangingEmail => StatusCode::UNAUTHORIZED,
                // DAL errors
                XamXamServiceError::XamXamDalError(_) => StatusCode::BAD_REQUEST,
                // JWT errors
                XamXamServiceError::JWTerror(_) => StatusCode::BAD_REQUEST,
                //Custom errors
                XamXamServiceError::CustomError(_) => StatusCode::BAD_REQUEST
            },
            _ => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}