use jwt_gang::claim_config::ClaimConfiguration;
use mailgang::curl_mail::Mailer;
use redis::Connection;
use crate::viewmodels::new_user::NewUser;
use xam_xam_dal::models::user::InsertableUser;
use xam_xam_dal::diesel::PgConnection;
use xam_xam_dal::repo::user;
use xam_xam_dal::util::get_hash;
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
    let html_text = format!(
        r#"
        <h1>Hello dear potential user</h1>
        </br>
        <p>Use this token to create your account: {}.</p>
        "#,&token
    );
    mailer.send_mail(email, "Token for user account createion", "", &html_text)?;
    Ok(())
}