use actix_web::{dev::HttpResponseBuilder, error, http::header, http::StatusCode, HttpResponse};
use jwt_gang::claim_error::JwtCustomError;
use std::{error::Error, fmt};
use xam_xam_id_bll::{err::XamXamServiceError, XamXamError};

#[derive(Debug, Clone)]
pub enum XamXamWebError {
    //Service related error
    ServiceError(XamXamServiceError),
    //DB related error
    CouldNotGetRedisConnection,
    CouldNotGetPostGresConnection,
    //config related error
    JwtConfigIsNotThere,
    //HTTP related
    CredentialsNotPresent,
    //Custom errors
    CustomError(String),
}

impl fmt::Display for XamXamWebError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            //Service
            XamXamWebError::ServiceError(err) => write!(f, "{}", err),
            //DB related error
            XamXamWebError::CouldNotGetRedisConnection => {
                write!(f, "Could not get the redis connection from the redis pool")
            }
            XamXamWebError::CouldNotGetPostGresConnection => write!(
                f,
                "Could not get the postgres connection from the postgres pool"
            ),
            //config related error
            XamXamWebError::JwtConfigIsNotThere => write!(f, "Could not get the JWT config"),
            //HTTP related
            XamXamWebError::CredentialsNotPresent => {
                write!(f, "Credentials where not present in the request")
            }
            // Custom errors
            XamXamWebError::CustomError(e) => write!(f, "{}", e),
        }
    }
}

impl From<actix_web::error::ParseError> for XamXamWebError {
    fn from(err: actix_web::error::ParseError) -> Self {
        XamXamWebError::CustomError(err.to_string())
    }
}

impl From<actix_web::Error> for XamXamWebError {
    fn from(err: actix_web::Error) -> Self {
        error!("{}", err);
        XamXamWebError::CustomError(err.to_string())
    }
}

impl From<&str> for XamXamWebError {
    fn from(err: &str) -> Self {
        error!("{}", err);
        XamXamWebError::CustomError(err.to_string())
    }
}

impl From<xam_xam_id_bll::err::XamXamServiceError> for XamXamWebError {
    fn from(err: xam_xam_id_bll::err::XamXamServiceError) -> Self {
        error!("{}", err);
        XamXamWebError::ServiceError(err)
    }
}

impl From<jwt_gang::claim_error::JwtCustomError> for XamXamWebError {
    fn from(err: jwt_gang::claim_error::JwtCustomError) -> Self {
        error!("{}", err);
        XamXamWebError::ServiceError(XamXamServiceError::JwtError(err))
    }
}

impl Error for XamXamWebError {}

impl XamXamWebError {
    fn show_public_error(&self) -> String {
        if let XamXamWebError::ServiceError(service_err) = &self {
            if let XamXamServiceError::XamXamDalError(dal_err) = service_err {
                return match dal_err {
                    XamXamError::EmailNotCorrectFormat => "Email is not in the correct form",
                    XamXamError::EmailIsEmpty => "Email cannot be empty",
                    XamXamError::EmailIsAlreadyTaken => "User email is already taken",
                    XamXamError::EmailAndPasswordIsEmpty => {
                        "Email and password can't be equal to nothing"
                    }
                    XamXamError::PasswordIsEmpty => "Password cannot be empty",
                    XamXamError::PasswordAndPasswordConfirmedNotEqual => {
                        "Password and confirmed password aren't the same"
                    }
                    XamXamError::UserNotFound => "User cannot be found",
                    XamXamError::UserAlreadyPresent => "User is already present",
                    XamXamError::UserIsNotPresent => "User is not present",
                    XamXamError::PasswordIsNotCorrect => "Given password is wrong",
                    _ => "An internal error happened",
                }
                .to_string();
            }
            if let XamXamServiceError::JwtError(err_jwt) = service_err {
                return match err_jwt {
                    JwtCustomError::TokenIsInvalid => "Token was invalid",
                    JwtCustomError::SignatureHasExpired => "Token has expired",
                    _ => "An internal error happened",
                }
                .to_string();
            }
            return match service_err {
                XamXamServiceError::TokenNotCorrectForUserCreation => {
                    "Token that was given is not correct, to create a new user"
                }
                XamXamServiceError::TokenNotCorrectForForgottenPwd => {
                    "Token that was given is not right, to change the forgotten password"
                }
                XamXamServiceError::TokenNotCorrectForChangingEmail => {
                    "Token that was given is not right, to change the email"
                }
                XamXamServiceError::TokenHasNotCorrectLength => {
                    "Token that was given doesn't have the right lenght"
                }
                _ => "An internal error happened",
            }
            .to_string();
        }
        match self {
            XamXamWebError::CredentialsNotPresent => "Credentials where not present in the request",
            _ => "An internal error happened",
        }
        .to_string()
    }
}

impl error::ResponseError for XamXamWebError {
    fn error_response(&self) -> HttpResponse {
        error!("{}", self.show_public_error());
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/plain")
            .body(self.show_public_error())
    }

    fn status_code(&self) -> StatusCode {
        match &*self {
            XamXamWebError::ServiceError(service_err) => match service_err {
                //User related error
                XamXamServiceError::UserAlreadyInRedisDb => StatusCode::UNAUTHORIZED,
                XamXamServiceError::TokenNotCorrectForUserCreation => StatusCode::UNAUTHORIZED,
                XamXamServiceError::TokenNotCorrectForForgottenPwd => StatusCode::UNAUTHORIZED,
                XamXamServiceError::TokenNotCorrectForChangingEmail => StatusCode::UNAUTHORIZED,
                XamXamServiceError::TokenHasNotCorrectLength => StatusCode::UNAUTHORIZED,
                // DAL errors
                XamXamServiceError::XamXamDalError(_) => StatusCode::BAD_REQUEST,
                // JWT errors
                XamXamServiceError::JwtError(_) => StatusCode::BAD_REQUEST,
                //Custom errors
                XamXamServiceError::CustomError(_) => StatusCode::BAD_REQUEST,
            },
            XamXamWebError::CredentialsNotPresent => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
