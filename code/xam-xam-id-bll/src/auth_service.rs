use jwt_gang::claim_config::ClaimConfiguration;
use mailgang::curl_mail::Mailer;
use redis::Connection;
use crate::viewmodels::new_user::NewUser;
use crate::viewmodels::email::EmailHolder;
use crate::viewmodels::password::PasswordHolder;
use crate::viewmodels::forgot_pwd::ForgottenPassword;
use xam_xam_dal::models::user::{InsertableUser,User};
use xam_xam_dal::diesel::PgConnection;
use xam_xam_dal::repo::user;
use xam_xam_common::util::{get_hash,control_email};
use xam_xam_dal::basic_user_info::BasicUserInfo;
use crate::err::XamXamServiceError;
use xam_xam_dal::err::XamXamError;

/**
 * Function that is used to control if a user is present in the postgresql or redis database, this will return an error. If after this it finds nothing a token which last for a certain time will be added to the redis database and a mail will be send to the user who wants to create an account with a token within.
*/
pub fn introduce_user_creation_demand(redis_conn : &mut Connection, db_conn : &PgConnection, mailer : Mailer, email : &str) -> Result<(),XamXamServiceError> {
    if user::user_exists_by_email(db_conn, email)? {
        return Err(XamXamError::UserAlreadyPresent.into())
    }
    if redis::cmd("EXISTS").arg(email).query::<bool>(redis_conn)? {
        return Err(XamXamServiceError::UserAlreadyInRedisDB)
    }
    let token = get_hash(4);
    redis::pipe()
    .cmd("SET").arg(email).arg(&token).ignore()
    .cmd("EXPIRE").arg(email).arg(600).ignore()
    .query(redis_conn)?;
    let clear_text = format!(
        r#"
        Hello dear potential user

        Use this token to create your account: {}.
        "#,&token
    );
    let html_text = format!(
        r#"
        <h1>Hello dear potential user</h1>
        </br>
        <p>Use this token to create your account: {}.</p>
        "#,&token
    );
    mailer.send_mail(email, "Token for user account createion", &clear_text, &html_text)?;
    Ok(())
}

/**
 * Function used to create an user in the database. It will check the given token that equal to the one found in the redis database. Passord and their confirmation needs to be equal to eacht other.
*/
pub fn create_user(redis_conn : &mut Connection, db_conn : &PgConnection, model : &NewUser) -> Result<(),XamXamServiceError> {
    let token = redis::cmd("GET").arg(model.get_email()).query::<String>(redis_conn)?;
    if model.get_password() != model.get_password_confirmed() {
        return Err(XamXamError::PasswordAndPasswordConfirmedNotEqual.into())
    }
    if token != model.get_token() {
        return Err(XamXamServiceError::TokenNotCorrectForUserCreation)
    }
    user::insert_user(db_conn, &InsertableUser::new(model.get_email(), model.get_password())?)?;
    Ok(())
}

/** 
 * Function used to change the email of the given user id. The email must comply with the email regex or it will be rejected.
*/
pub fn change_email(db_conn : &PgConnection, user_id : i32, model : EmailHolder) -> Result<(),XamXamServiceError> {
    if !control_email(model.get_email()) {
        return Err(XamXamError::EmailNotCorrectFormat.into())
    }
    user::change_email(db_conn, user_id, model.get_email())?;
    Ok(())
}

/**
 * Function used to change the password of the given user id. The given password and its confirmation equals to each other or an error will be returned.
*/
pub fn change_pwd (db_conn : &PgConnection, user_id : i32, model : &PasswordHolder)-> Result<(),XamXamServiceError> {
    if model.get_password().is_empty() {
        return Err(XamXamError::PasswordIsEmpty.into())
    }
    if model.get_password() != model.get_password_confirmed() {
        return Err(XamXamError::PasswordAndPasswordConfirmedNotEqual.into())
    }
    user::change_password(db_conn, user_id, model.get_password())?;
    Ok(())
}

/**
 * Function that sends a mail with a token that the user uses to change his forgotten password. The email associated with the user wanting to change is used as a key appended with the word ':pwd-token'.
*/
pub fn send_token_forgotten_pwd(redis_conn : &mut Connection, db_conn : &PgConnection, mailer : Mailer, model : EmailHolder) -> Result<(),XamXamServiceError> {
    let user : User = user::get_user_by_mail(db_conn, model.get_email())?.ok_or_else(|| XamXamServiceError::from(XamXamError::UserIsNotPresent))?;
    if redis::cmd("EXISTS").arg(user.id).query::<bool>(redis_conn)? {
        return Err(XamXamServiceError::UserAlreadyInRedisDB)
    }
    let token = get_hash(6);
    redis::pipe()
    .cmd("SET").arg(user.id).arg(&token).ignore()
    .cmd("EXPIRE").arg(user.id).arg(600).ignore()
    .query(redis_conn)?;
    let clear_text = format!(
        r#"
        Hello dear user

        Use this token to put in the form to change you're password you have forgotten: {}.
        "#,&token
    );
    let html_text = format!(
        r#"
        <h1>Hello dear potential user</h1>
        </br>
        <p>Use this token to put in the form to change you're password you have forgotten: {}.</p>
        "#,&token
    );
    mailer.send_mail(model.get_email(), "Token to change forgotten password", &clear_text, &html_text)?;
    Ok(())
}

/**
 * Function that is used to change the password of a person that has forgotten theirs. It compares the given token and compares with what is found inside the redis database and if equal the change is then made.
*/
pub fn change_forgotten_pwd(redis_conn : &mut Connection, db_conn : &PgConnection, model : &ForgottenPassword) -> Result<(),XamXamServiceError> {
    if model.get_password().is_empty() {
        return Err(XamXamError::PasswordIsEmpty.into())
    }
    if model.get_password() != model.get_password_confirmed() {
        return Err(XamXamError::PasswordAndPasswordConfirmedNotEqual.into())
    }
    let person : User = user::get_user_by_mail(db_conn, model.get_email())?.ok_or_else(|| XamXamServiceError::from(XamXamError::UserIsNotPresent))?;
    let token : String = redis::cmd("GET").arg(person.id).query::<String>(redis_conn)?;
    if token != model.get_token() {
        return Err(XamXamServiceError::TokenNotCorrectForForgottenPwd)
    }
    user::change_password(db_conn, person.id, model.get_password())?;
    redis::cmd("DEL").arg(person.id).query(redis_conn)?;
    Ok(())
}

/**
 * Authenticates a user password and returns if the result is right or not. It will return if right a jwt token string, that the user can use to authenticate and be authorized to do his operations, otherwise an error is thrown out.
*/
pub fn authenthicate_get_token(db_conn : &PgConnection, claim_config : &ClaimConfiguration, email : &str, pwd : &str) -> Result<String, XamXamServiceError> {
    if pwd.is_empty() {
        return Err(XamXamError::PasswordIsEmpty.into())
    }
    let person : User = user::get_user_by_mail(db_conn, email)?.ok_or_else(|| XamXamServiceError::from(XamXamError::UserIsNotPresent))?;
    if person.verify_pwd(pwd)? {
        return Err(XamXamError::PasswordIsNotCorrect.into())
    }
    Ok(claim_config.token_from_claim(&claim_config.create_claim(&person.id.to_string())?)?)
}

/**
 * returns the struct containing basic information of the user with the give id.
*/
pub fn get_basic_information(db_conn : &PgConnection, user_id : i32) -> Result<BasicUserInfo,XamXamServiceError> {
    Ok(user::get_information_from_id(db_conn, user_id)?)
}

/**
 * Function that controls the given token and returns a new token if the previous was valid.
*/
pub fn renew_token(claim_config : &ClaimConfiguration, token : &str) -> Result<String, XamXamServiceError> {
    let claim = claim_config.decode_token(token)?;
    Ok(claim_config.token_from_claim(&claim_config.create_claim(&claim.claims.get_subject())?)?)
}