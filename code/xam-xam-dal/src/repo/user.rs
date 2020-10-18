use diesel::PgConnection;
use crate::schema::users::dsl::*;
use crate::models::user::{InsertableUser, User};
use crate::err::XamXamError;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use crate::diesel::query_dsl::filter_dsl::FindDsl;
use crate::diesel::query_dsl::filter_dsl::FilterDsl;
use crate::diesel::query_dsl::select_dsl::SelectDsl;
use crate::util::{control_email,get_hash};
use crate::const_values;
use argon2::Config;
use crate::basic_user_info::BasicUserInfo;
use diesel::sql_types::Integer;

/**
 * Inserts a user in a database
 */
pub fn insert_user(conn : &PgConnection, user : &InsertableUser) -> Result<(),XamXamError> {
    match diesel::insert_into(users).values(user).get_result::<User>(conn) {
        Ok(user) => {
            info!("User has succesfully been inserted, with email {} and with user id {}", &user.email,&user.id);
            Ok(())
        },
        Err(e) => Err(e.into())
    }
}

/**
 * Returns a user based on his email
 */
pub fn get_user_by_mail(conn : &PgConnection, user_email : &str) -> Result<User,XamXamError> {
    Ok(
        users.filter(email.eq(user_email)).first::<User>(conn)?
    )
}

/**
 * Function that is used to confirm a user, mostly through the email. The result can if succeeded return a boolean, true means a row has changed otherwhise a false will be given.
 */
pub fn confirm_user(conn : &PgConnection, user_id : i32) -> Result<bool, XamXamError> {
    match diesel::update(users.find(user_id)).set(email_confirmed.eq(true)).execute(conn) {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                info!("The user with the id {} has succesfully been confirmed.", user_id);
                Ok(true)
            } else {
                info!("The user with the id {} has succesfully couldn't be confirmed.", user_id);
                Ok(false)
            }
        },
        Err(e) => Err(e.into())
    }
}

/**
 * Function that is used to change the email of an user. The result can if succeeded return a boolean, true means a row has changed otherwhise a false will be given.
 */
pub fn change_email(conn : &PgConnection, user_id : i32, new_user_email : &str) -> Result<bool,XamXamError> {
    if !control_email(new_user_email) {
        return Err(XamXamError::EmailNotCorrectFormat)
    }
    match diesel::update(users.find(user_id)).set(email.eq(new_user_email)).execute(conn) {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                info!("The user with the id {} has succesfully changed its email", user_id);
                Ok(true)
            } else {
                info!("The user with the id {} has succesfully couldn't change his email", user_id);
                Ok(false)
            }
        },
        Err(e) => Err(e.into())
    }
}

/**
 * Function that is used to change the password of an user. The result can if succeeded return a boolean, true means a row has changed otherwhise a false will be given.
 */
pub fn change_password(conn : &PgConnection, user_id : i32, new_user_pwd : &str) -> Result<bool,XamXamError> {
    if new_user_pwd.is_empty() {
        return Err(XamXamError::PasswordIsEmpty)
    }
    let new_salt : String = get_hash(const_values::SALT_LENGTH);
    let hashed_pwd : String = match argon2::hash_encoded(new_user_pwd.as_bytes(), new_salt.as_bytes(), &Config::default()) {
        Ok(hash) => hash,
        Err(_) => return Err(XamXamError::PasswordCannotBeMade) 
    };
    match diesel::update(users.find(user_id)).set((password_hash.eq(&hashed_pwd),salt.eq(&new_salt))).execute(conn) {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                info!("The user with the id {} has succesfully changed its password", user_id);
                Ok(true)
            } else {
                info!("The user with the id {} has succesfully couldn't change his password", user_id);
                Ok(false)
            }
        },
        Err(e) => Err(e.into())
    }
}

/**
 * Function that is used to delete an user based on its user id. The result could return a bool, it is true when a user has been deleted and a false when it was not there to begin with.
 */
pub fn delete_user(conn : &PgConnection, user_id : i32) -> Result<bool,XamXamError> {
    match diesel::delete(users.find(user_id)).execute(conn) {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                info!("The user with the id {} has succesfully been deleted", user_id);
                Ok(true)
            } else {
                info!("The user with the id {} has succesfully couldn't be deleted", user_id);
                Ok(false)
            }
        },
        Err(e) => Err(e.into())
    }
}

/**
 * Returns if a user is verified or not.
 */
pub fn is_user_confirmed(conn : &PgConnection, user_id : i32) -> Result<bool,XamXamError> {
    Ok(users.find(user_id).select(email_confirmed).get_result(conn)?)
}

/**
 * Function that is used to show statistics that user would want.
 */
pub fn get_information_from_id(conn : &PgConnection, user_id : i32) -> Result<BasicUserInfo, XamXamError> {
    let result : BasicUserInfo = diesel::sql_query(r#"select count(s.id) as amount_storage,
    count(pi.id) as amount_product,
    min(pi.peremption_date) as min_bederf,
    max(pi.peremption_date) as max_bederf
    from storages s left join products pi on s.ID = pi.id where s.user_id = ?
    "#).bind::<Integer,_>(user_id).get_result(conn)?;
    Ok(result)
}