use jwt_gang::claim_config::ClaimConfiguration;
use mailgang::mailer_gang::Mailer;
use mailgang::mail_struct::Report;
use r2d2_redis::redis;
use crate::viewmodels::new_user::NewUser;
use crate::viewmodels::email::EmailHolder;
use crate::viewmodels::new_email::NewEmailHolder;
use crate::viewmodels::password::PasswordHolder;
use crate::viewmodels::forgot_pwd::ForgottenPassword;
use xam_xam_dal::models::user::{InsertableUser,User};
use xam_xam_dal::repo::user;
use xam_xam_common::util::{get_hash,control_email};
use xam_xam_dal::models::basic_user_info::BasicUserInfo;
use crate::err::XamXamServiceError;
use xam_xam_dal::err::XamXamError;
use crate::{RCon,PgCon};
use std::ops::DerefMut;
use crate::viewmodels::product_unit::ProductUnits;

/**
 * ============================================================ USER CREATION ============================================================
 */
/**
 * Function that is used to control if a user is present in the postgresql or redis database, this will return an error. If after this it finds nothing a token which last for a certain time will be added to the redis database and a mail will be send to the user who wants to create an account with a token within.
*/
pub fn introduce_user_creation_demand(redis_conn : &mut RCon, db_conn : &PgCon, mailer : &Mailer, email : &str) -> Result<(),XamXamServiceError> {
    if user::user_exists_by_email(db_conn, email)? {
        info!("email {} already existed in the postgres database", email);
        return Err(XamXamError::UserAlreadyPresent.into())
    }
    if r2d2_redis::redis::cmd("EXISTS").arg(email).query::<bool>(redis_conn.deref_mut())? {
        info!("email {} already existed in the redis database", email);
        return Err(XamXamServiceError::UserAlreadyInRedisDB)
    }
    let token = get_hash(4);
    r2d2_redis::redis::pipe()
    .cmd("SET").arg(&token).arg(email).ignore()
    .cmd("EXPIRE").arg(&token).arg(600).ignore()
    .query(redis_conn.deref_mut())?;
    let clear_text = format!(
        r#"
        Hello dear potential user

        Use this token to create your account: {}.
        "#,&token
    );
    mailer.send_email(Report::new(email,"", "Token for user account creation", &clear_text)?)?;
    Ok(())
}

/**
 * Function used to create an user in the database. It will check the given token that equal to the one found in the redis database. Passord and their confirmation needs to be equal to each other. Also checks that the token has the lenght of 6.
*/
pub fn create_user(redis_conn : &mut RCon, db_conn : &PgCon, model : &NewUser) -> Result<(),XamXamServiceError> {
    if user::user_exists_by_email(db_conn, model.get_email())? {
        return Err(XamXamError::UserAlreadyPresent.into())
    }
    if model.get_password() != model.get_password_confirmed() {
        return Err(XamXamError::PasswordAndPasswordConfirmedNotEqual.into())
    }
    if model.get_token().len() != 4 {
        return Err(XamXamServiceError::TokenHasNotCorrectLength);
    }
    let email : Option<String> = match redis::cmd("GET").arg(model.get_token().to_owned()).query(redis_conn.deref_mut()) {
        Ok(res) => res,
        Err(e) => {
            error!("{}",&e);
            return Err(XamXamServiceError::from(e))
        }
    };
    if email.unwrap_or_else(|| "".to_string()) != model.get_email() {
        return Err(XamXamServiceError::TokenNotCorrectForUserCreation)
    }
    redis::cmd("DEL").arg(model.get_token()).query(redis_conn.deref_mut())?;
    user::insert_user(db_conn, &InsertableUser::new(model.get_email(), model.get_password())?)?;
    Ok(())
}

/**
 * ============================================================ CHANGING EMAIL ============================================================
 */
/**
 * Introduces a demand to change email, to confirm this change a token is sent to the email you want to change to. The email the user wants to change to should not yet exist in the postgres or redis database or it will return an error.
 */
pub fn request_email_change(redis_conn : &mut RCon, db_conn : &PgCon, model : &EmailHolder, user_id : i32, mailer : &Mailer)-> Result<(),XamXamServiceError> {
    let current_email : String = user::get_email_by_id(db_conn, user_id)?;
    info!("Email of current user has been asked");
    if !control_email(model.get_email()) {
        return Err(XamXamError::EmailNotCorrectFormat.into())
    }
    if user::user_exists_by_email(db_conn, model.get_email())? {
        return Err(XamXamError::EmailIsAlreadyTaken.into())
    }
    let token = get_hash(4);
    redis::pipe()
    .cmd("SET").arg(&token).arg(model.get_email()).ignore()
    .cmd("EXPIRE").arg(&token).arg(600).ignore()
    .query(redis_conn.deref_mut())?;
    let clear_text = format!(
        r#"
        Hello dear potential user

        Use this token to change your user account email from {} to {}: {}.
        "#,&current_email,model.get_email(),&token
    );
    mailer.send_email(Report::new(&model.get_email(),"", "Token for changing emails", &clear_text)?)?;
    Ok(())
}

/**
 * Function used to change the email of the given user id. The email must comply with the email regex or it will be rejected. It will also need a token you get from your the email you want to change to.
*/
pub fn change_email(redis_conn : &mut RCon, db_conn : &PgCon, user_id : i32, model : &NewEmailHolder) -> Result<(),XamXamServiceError> {
    if model.get_token().len() != 4 {
        return Err(XamXamServiceError::TokenHasNotCorrectLength);
    }
    let email = redis::cmd("GET").arg(model.get_token()).query::<String>(redis_conn.deref_mut())?;
    if email != model.get_email() {
        return Err(XamXamServiceError::TokenNotCorrectForChangingEmail)
    }
    if !control_email(model.get_email()) {
        return Err(XamXamError::EmailNotCorrectFormat.into())
    }
    if user::user_exists_by_email(db_conn, model.get_email())? {
        return Err(XamXamError::UserAlreadyPresent.into())
    }
    user::change_email(db_conn, user_id, model.get_email())?;
    redis::cmd("DEL").arg(model.get_email()).query(redis_conn.deref_mut())?;
    Ok(())
}

/**
 * ========================================================== CHANGING PASSWORD ============================================================
 */
/**
 * Function used to change the password of the given user id. The given password and its confirmation equals to each other or an error will be returned.
*/
pub fn change_pwd (db_conn : &PgCon, model : &PasswordHolder, email : &str, pwd : &str)-> Result<(),XamXamServiceError> {
    if pwd.is_empty() {
        return Err(XamXamError::PasswordIsEmpty.into())
    }
    let person : User = user::get_user_by_mail(db_conn, email)?.ok_or_else(|| XamXamServiceError::from(XamXamError::UserIsNotPresent))?;
    if !person.verify_pwd(pwd)? {
        return Err(XamXamError::PasswordIsNotCorrect.into())
    }
    user::change_password(db_conn, person.id, model.get_password())?;
    Ok(())
}

/**
 * ============================================================ FORGOTTEN PASSWORD ============================================================
 */
/**
 * Function that sends a mail with a token that the user uses to change his forgotten password. The email associated with the user wanting to change is used as a key appended with the word ':pwd-token'.
*/
pub fn send_token_forgotten_pwd(redis_conn : &mut RCon, db_conn : &PgCon, mailer : &Mailer, model : &EmailHolder) -> Result<(),XamXamServiceError> {
    if !control_email(model.get_email()) {
        return Err(XamXamError::EmailNotCorrectFormat.into())
    }
    if !user::user_exists_by_email(db_conn, model.get_email())? {
        return Err(XamXamError::UserNotFound.into())
    }
    let token = get_hash(5);
    redis::pipe()
    .cmd("SET").arg(&token).arg(model.get_email()).ignore()
    .cmd("EXPIRE").arg(&token).arg(600).ignore()
    .query(redis_conn.deref_mut())?;
    let clear_text = format!(
        r#"
        Hello dear user

        Use this token to put in the form to change you're password you have forgotten: {}.
        "#,&token
    );
    mailer.send_email(Report::new(model.get_email(), "", "Token to change forgotten password", &clear_text)?)?;
    Ok(())
}

/**
 * Function that is used to change the password of a person that has forgotten theirs. It compares the given token and compares with what is found inside the redis database and if equal the change is then made.
*/
pub fn change_forgotten_pwd(redis_conn : &mut RCon, db_conn : &PgCon, model : &ForgottenPassword) -> Result<(),XamXamServiceError> {
    if model.get_password().is_empty() {
        return Err(XamXamError::PasswordIsEmpty.into())
    }
    if model.get_password() != model.get_password_confirmed() {
        return Err(XamXamError::PasswordAndPasswordConfirmedNotEqual.into())
    }
    if model.get_token().len() != 5 {
        return Err(XamXamServiceError::TokenHasNotCorrectLength);
    }
    let person : User = user::get_user_by_mail(db_conn, model.get_email())?.ok_or_else(|| XamXamServiceError::from(XamXamError::UserIsNotPresent))?;
    let email : String = redis::cmd("GET").arg(model.get_token()).query::<String>(redis_conn.deref_mut())?;
    if email != model.get_email() {
        return Err(XamXamServiceError::TokenNotCorrectForForgottenPwd)
    }
    user::change_password(db_conn, person.id, model.get_password())?;
    redis::cmd("DEL").arg(person.id).query(redis_conn.deref_mut())?;
    Ok(())
}

/**
 * ============================================================ JWT TOKENS ============================================================
 */
/**
 * Authenticates a user password and returns if the result is right or not. It will return if right a jwt token string, that the user can use to authenticate and be authorized to do his operations, otherwise an error is thrown out.
*/
pub fn authenthicate_get_token(db_conn : &PgCon, claim_config : &ClaimConfiguration, email : &str, pwd : &str) -> Result<String, XamXamServiceError> {
    if pwd.is_empty() {
        return Err(XamXamError::PasswordIsEmpty.into())
    }
    let person : User = user::get_user_by_mail(db_conn, email)?.ok_or_else(|| XamXamServiceError::from(XamXamError::UserIsNotPresent))?;
    if !person.verify_pwd(pwd)? {
        return Err(XamXamError::PasswordIsNotCorrect.into())
    }
    info!("Person with email {} has the good password.",email);
    let claim = claim_config.create_claim(&person.id.to_string())?;
    Ok(claim_config.token_from_claim(&claim)?)
}

/**
 * Function that controls the given token and returns a new token if the previous was valid.
*/
pub fn renew_token(claim_config : &ClaimConfiguration, token : &str) -> Result<String, XamXamServiceError> {
    Ok(
        claim_config.token_from_claim(
            &claim_config.create_claim(
                &claim_config.decode_token(token)?.claims.get_subject()
            )?
        )?
    )
}

/**
 * ==================================================== GET BASIC INFO USER =======================================================
 */
/**
 * returns the struct containing basic information of the user with the give id.
*/
pub fn get_basic_information(db_conn : &PgCon, user_id : i32) -> Result<BasicUserInfo,XamXamServiceError> {
    Ok(user::get_information_from_id(db_conn, user_id)?)
}

/**
 * ==================================================== RECENT PRODUCTS ==================================
 */
pub fn get_five_first_products(conn : &PgCon, user_id : i32) -> Result<ProductUnits, XamXamServiceError> {
    Ok(user::get_five_first_products(conn, user_id)?.into())
}

/**
 * ==================================================== GET EMAIL ==============================================
*/
pub fn get_email_from_id(user_id : i32, db_conn : &PgCon) -> Result<EmailHolder, XamXamServiceError> {
    Ok(EmailHolder::from(user::get_email_by_id(db_conn, user_id)?))
}

/**
 * ================================================= DELETE PROFILE =======================================================
 */
pub fn delete_user(db_conn : &PgCon, email : &str, pwd : &str) -> Result<(),XamXamServiceError> {
    if pwd.is_empty() {
        return Err(XamXamError::PasswordIsEmpty.into())
    }
    let person : User = user::get_user_by_mail(db_conn, email)?.ok_or_else(|| XamXamServiceError::from(XamXamError::UserIsNotPresent))?;
    if !person.verify_pwd(pwd)? {
        return Err(XamXamError::PasswordIsNotCorrect.into())
    }
    user::delete_user(db_conn,person.id)?;
    Ok(())
}