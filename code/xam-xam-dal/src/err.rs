use std::{error::Error, fmt};
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub enum XamXamError {
    //User-related error
    EmailNotCorrectFormat,
    EmailIsEmpty,
    EmailIsAlreadyTaken,
    IdIsAlreadyTaken,
    EmailAndPasswordIsEmpty,
    PasswordIsNotCorrect,
    PasswordIsEmpty,
    PasswordCannotBeMade,
    PasswordAndPasswordConfirmedNotEqual,
    UserNotFound,
    UserCannotBeAdded,
    UserAlreadyPresent,
    UserIsNotPresent,
    UserDeleteFailed,
    UserCannotBeUpdated,
    //Storage related error
    StorageNameIsEmpty,
    // Storage related errors
    ProductNameIsEmpty,
    //Token related error
    SubjectOfTokenIsEmpty,
    TokenCannotBeMadeFromClaim,
    TokenIsEmpty,
    TokenIsInvalid,
    IssuerIsInvalid,
    SignatureHasExpired,
    //Email and SMTP related errors
    SmtpDomainNotGood,
    CouldNotSendEmail,
    //Custom errors
    CustomError(String)
}

impl fmt::Display for XamXamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            //User-related error
            XamXamError::EmailNotCorrectFormat => write!(f,"Email is not in the correct form"),
            XamXamError::EmailIsEmpty => write!(f,"Email cannot be empty"),
            XamXamError::EmailIsAlreadyTaken => write!(f,"User email is already taken"),
            XamXamError::IdIsAlreadyTaken => write!(f,"User id is already taken"),
            XamXamError::EmailAndPasswordIsEmpty => write!(f,"Email and password can't be equal to nothing"),
            XamXamError::PasswordIsNotCorrect => write!(f,"Password is not right"),
            XamXamError::PasswordIsEmpty => write!(f,"Password cannot be empty"),
            XamXamError::PasswordCannotBeMade => write!(f,"Password couldn't be made"),
            XamXamError::PasswordAndPasswordConfirmedNotEqual => write!(f,"Password and confirmed password aren't the same"),
            XamXamError::UserNotFound => write!(f,"User cannot be found"),
            XamXamError::UserCannotBeAdded => write!(f,"User cannot be added"),
            XamXamError::UserAlreadyPresent => write!(f,"User is already present"),
            XamXamError::UserIsNotPresent => write!(f,"User is not present"),
            XamXamError::UserDeleteFailed => write!(f,"The user's password wasn't correct or delete confirmation was not set to true"),
            XamXamError::UserCannotBeUpdated => write!(f,"User cannot be updated."),
            //Storage related error
            XamXamError::StorageNameIsEmpty => write!(f,"The name of a storage cannot be empty"),
            // Storage related errors
            XamXamError::ProductNameIsEmpty => write!(f,"The name of a product cannot be emtpy"),
            //Token related error
            XamXamError::SubjectOfTokenIsEmpty => write!(f,"The subject is empty"),
            XamXamError::TokenCannotBeMadeFromClaim => write!(f,"Couldn't create a token out of a claim"),
            XamXamError::TokenIsEmpty => write!(f,"Token cannot be emtpy"),
            XamXamError::TokenIsInvalid => write!(f,"Token is invalid"),
            XamXamError::IssuerIsInvalid => write!(f,"Issuer is invalid"),
            XamXamError::SignatureHasExpired => write!(f,"Signature has expired"),
            //Email and SMTP related errors
            XamXamError::SmtpDomainNotGood => write!(f,"Stmp domain is not good"),
            XamXamError::CouldNotSendEmail => write!(f,"Could not send the email throught the smtp transport"),
            // Custom errors
            XamXamError::CustomError(e) => write!(f,"{}",e)
        }
    }
}

impl Error for XamXamError { }