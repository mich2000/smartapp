use xam_xam_dal::diesel::PgConnection;
use jwt_gang::claim_config::ClaimConfiguration;
use xam_xam_dal::repo::user;
use mailgang::curl_mail::Mailer;
use redis::Connection;
use crate::viewmodels::new_user::NewUser;
use xam_xam_dal::models::user::InsertableUser;
use xam_xam_dal::util::get_hash;

use crate::err::XamXamServiceError;
use xam_xam_dal::err::XamXamError;

pub fn introduce_user_creation_demand(redis_conn : &mut Connection, db_conn : &PgConnection, mailer : Mailer, email : &str) -> Result<(),XamXamServiceError> {
    if user::get_user_by_mail(db_conn, email)?.is_some() {
        return Err(XamXamError::UserAlreadyPresent.into())
    }
    redis::cmd("SET").arg(email).arg(get_hash(4)).query(redis_conn)?;
    redis::cmd("EXPIRE").arg(email).arg(600).query(redis_conn)?;
    //TODO send mail with mailer variable for token to create user account.
    Ok(())
}