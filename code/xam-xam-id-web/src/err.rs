use std::{error::Error, fmt};
use serde::{Serialize,Deserialize};
use xam_xam_id_bll::err::XamXamServiceError;

#[derive(Debug,Serialize,Deserialize)]
pub enum XamXamWebError {
    //Service related error
    ServiceError(XamXamServiceError),
    //Custom errors
    CustomError(String)
}

impl fmt::Display for XamXamWebError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            //Service
            XamXamWebError::ServiceError(err) => write!(f,"{}",err),
            // Custom errors
            XamXamWebError::CustomError(e) => write!(f,"{}",e)
        }
    }
}

impl From<&str> for XamXamWebError { 
    fn from (err : &str) -> Self { XamXamWebError::CustomError(err.to_string()) } 
}

impl From<xam_xam_id_bll::err::XamXamServiceError> for XamXamWebError {
    fn from(err: xam_xam_id_bll::err::XamXamServiceError) -> Self {
        XamXamWebError::ServiceError(err)
    }
}

impl Error for XamXamWebError { }

impl xam_xam_common::err_trait::PublicErrorTrait for XamXamWebError {
    fn show_public_error(&self) -> String {
        if let XamXamWebError::ServiceError(value) = &self {
            return value.show_public_error()
        }
        match self {
            _ => "An internal error happened"
        }.to_string()
    }
}

use actix_web::{
    dev::HttpResponseBuilder, error, http::header, http::StatusCode, HttpResponse
};

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

impl error::ResponseError for XamXamWebError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .json(
                XamActixError::new(self)
            )
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