use std::{error::Error, fmt};
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize,Clone)]
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
    UserNotVerified,
    //Storage related error
    StorageNameIsEmpty,
    // Storage related errors
    ProductNameIsEmpty,
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
            XamXamError::UserNotVerified => write!(f,"User has to be verified"),
            //Storage related error
            XamXamError::StorageNameIsEmpty => write!(f,"The name of a storage cannot be empty"),
            // Storage related errors
            XamXamError::ProductNameIsEmpty => write!(f,"The name of a product cannot be emtpy"),
            // Custom errors
            XamXamError::CustomError(e) => write!(f,"{}",e)
        }
    }
}

impl From<diesel::result::Error> for XamXamError {
    fn from(error: diesel::result::Error) -> Self {
        XamXamError::CustomError(format!("{}", error))
    }
}

impl From<bcrypt::BcryptError> for XamXamError {
    fn from(error : bcrypt::BcryptError) -> Self {
        XamXamError::CustomError(format!("{}",error))
    }
}

impl Error for XamXamError { }