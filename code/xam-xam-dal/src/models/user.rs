use crate::schema::*;
use crate::err::XamXamError;
use argon2::Config;
use crate::const_values;

/**
 * Struct that represents the basic user. This form of user is very simple.
*/
#[derive(Debug,Queryable, Identifiable,Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub email : String,
    pub password_hash : String,
    pub salt : String
}

impl User {
    pub fn verify_pwd(&self, pwd : &str) -> bool {
        if pwd.is_empty() {
            return false
        }
        argon2::verify_encoded(&self.password_hash, pwd.as_bytes()).unwrap()
    }
}

/**
 * Struct that will be inserted in the database.
 */
#[derive(Insertable,Debug)]
#[table_name = "users"]
pub struct InsertableUser {
    pub email : String,
    pub password_hash : String,
    pub salt : String
}

impl InsertableUser {
    /**
     * Method that returns InsertableUser struct in a Result, errors can be returned if:
     * * email is empty
     * * password is empty
     * * email isn't in the right format
     * * Hashing of password fails
     */
    pub fn new(email : &str, pwd : &str) -> Result<InsertableUser, XamXamError> {
        if email.is_empty() {
            return Err(XamXamError::EmailIsEmpty)
        }
        if pwd.is_empty() {
            return Err(XamXamError::PasswordIsEmpty)
        }
        if !xam_xam_common::util::control_email(email) {
            return Err(XamXamError::EmailNotCorrectFormat)
        }
        let hash : String = xam_xam_common::util::get_hash(const_values::SALT_LENGTH);
        let hashed_pwd : String = match argon2::hash_encoded(pwd.as_bytes(), hash.as_bytes(), &Config::default()) {
            Ok(hash) => hash,
            Err(_) => return Err(XamXamError::PasswordCannotBeMade) 
        };
        Ok(
            InsertableUser {
                email : email.to_string(),
                password_hash :  hashed_pwd,
                salt : hash
            }
        )
    }
}