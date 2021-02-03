use crate::err::XamXamError;
use crate::schema::*;
use bcrypt::{hash, verify, DEFAULT_COST};

/**
 * Struct that represents the basic user. This form of user is very simple.
*/
#[derive(Queryable, Identifiable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn verify_pwd(&self, pwd: &str) -> Result<bool, XamXamError> {
        if pwd.is_empty() {
            return Ok(false);
        }
        Ok(verify(pwd, &self.password_hash)?)
    }
}

/**
 * Struct that will be inserted in the database.
 */
#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct InsertableUser {
    pub email: String,
    pub password_hash: String,
}

impl InsertableUser {
    /**
     * Method that returns InsertableUser struct in a Result, errors can be returned if:
     * * email is empty
     * * password is empty
     * * email isn't in the right format
     * * Hashing of password fails
     */
    pub fn new(email: &str, pwd: &str) -> Result<InsertableUser, XamXamError> {
        if email.is_empty() {
            return Err(XamXamError::EmailIsEmpty);
        }
        if pwd.is_empty() {
            return Err(XamXamError::PasswordIsEmpty);
        }
        if !xam_xam_common::util::control_email(email) {
            return Err(XamXamError::EmailNotCorrectFormat);
        }
        let hashed_pwd: String = match hash(pwd, DEFAULT_COST) {
            Ok(hash) => hash,
            Err(_) => return Err(XamXamError::PasswordCannotBeMade),
        };
        Ok(InsertableUser {
            email: email.to_string(),
            password_hash: hashed_pwd,
        })
    }
}
