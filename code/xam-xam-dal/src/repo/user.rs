use crate::PgCon;
use crate::schema::users::dsl::*;
use crate::models::user::{InsertableUser, User};
use crate::err::XamXamError;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use crate::diesel::query_dsl::filter_dsl::FindDsl;
use crate::diesel::query_dsl::filter_dsl::FilterDsl;
use crate::diesel::query_dsl::select_dsl::SelectDsl;
use crate::diesel::OptionalExtension;
use xam_xam_common::util::{control_email};
use crate::basic_user_info::BasicUserInfo;
use bcrypt::{DEFAULT_COST, hash};

/**
 * Inserts a user in a database
 */
pub fn insert_user(conn : &PgCon, user : &InsertableUser) -> Result<(),XamXamError> {
    match diesel::insert_into(users).values(user).get_result::<User>(conn) {
        Ok(user) => {
            info!("User has succesfully been inserted, with email {} and with user id {}", &user.email,&user.id);
            Ok(())
        },
        Err(e) => Err(e.into())
    }
}

/**
 * Function that is used to know if a user with a specific id exists.
*/
pub fn user_exists_by_id(conn : &PgCon, user_id : i32) -> Result<bool,XamXamError> {
    Ok(
        users.filter(id.eq(user_id)).select(diesel::dsl::count_star()).get_result::<i64>(conn)? > 0
    )
}

/**
 * Function that is used to know if a user with a specific email exists.
*/
pub fn user_exists_by_email(conn : &PgCon, user_email : &str) -> Result<bool,XamXamError> {
    Ok(
        users.filter(email.eq(user_email)).select(diesel::dsl::count_star()).get_result::<i64>(conn)? > 0
    )
}

pub fn get_email_by_id(conn : &PgCon, user_id : i32) -> Result<String,XamXamError> {
    Ok(
        users.filter(id.eq(user_id)).select(email).get_result::<String>(conn)?
    )
}

/**
 * Returns a user based on his email
 */
pub fn get_user_by_mail(conn : &PgCon, user_email : &str) -> Result<Option<User>,XamXamError> {
    Ok(
        users.filter(email.eq(user_email)).get_result::<User>(conn).optional()?
    )
}

/**
 * Returns a user based on his id
 */
pub fn get_user_by_id(conn : &PgCon, user_id : i32) -> Result<Option<User>,XamXamError> {
    Ok(
        users.find(user_id).get_result::<User>(conn).optional()?
    )
}

/**
 * Function that is used to change the email of an user. The result can if succeeded return a boolean, true means a row has changed otherwhise a false will be given.
 */
pub fn change_email(conn : &PgCon, user_id : i32, new_user_email : &str) -> Result<bool,XamXamError> {
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
pub fn change_password(conn : &PgCon, user_id : i32, new_user_pwd : &str) -> Result<bool,XamXamError> {
    if new_user_pwd.is_empty() {
        return Err(XamXamError::PasswordIsEmpty)
    }
    let hashed_pwd : String = match hash(new_user_pwd,DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => return Err(XamXamError::PasswordCannotBeMade) 
    };
    match diesel::update(users.find(user_id)).set(password_hash.eq(&hashed_pwd)).execute(conn) {
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
pub fn delete_user(conn : &PgCon, user_id : i32) -> Result<bool,XamXamError> {
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
 * Function that is used to show statistics that user would want.
 */
pub fn get_information_from_id(conn : &PgCon, user_id : i32) -> Result<BasicUserInfo, XamXamError> {
    info!("id of basic user info {}",user_id);
    let result : BasicUserInfo = diesel::sql_query(format!("select u.email,count(s.id) as amount_storage, count(pi.id) as amount_product, min(pi.peremption_date) as min_bederf, max(pi.peremption_date) as max_bederf  from storages s  right join users u on s.user_id = u.id left join products pi on pi.storage_id = s.id where u.id = {} group by u.email",user_id)).get_result(conn)?;
    Ok(result)
}