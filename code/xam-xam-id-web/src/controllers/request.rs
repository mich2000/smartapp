use crate::db::{get_pg_conn, get_redis_conn};
use crate::err::XamXamWebError;
use crate::extractors::user_id::UserId;
use crate::{PgPool, RedisPool};
use actix_web::{post, web::Data, web::Json, HttpResponse};
use mailgang::mailer_gang::Mailer;
use xam_xam_id_bll::auth_service;
use xam_xam_id_bll::viewmodels::email::EmailHolder;
use xam_xam_id_bll::{PgCon, RCon};

/**
 * Route that is used to request a token received on the email, this token is then used to make a new user.
 */
#[post("/new/user")]
pub async fn request_new_user(
    redis_db: Data<RedisPool>,
    pg: Data<PgPool>,
    mailer: Data<Mailer>,
    model: Json<EmailHolder>,
) -> Result<HttpResponse, XamXamWebError> {
    let pg_conn: PgCon = get_pg_conn(pg)?;
    let mut r_conn: RCon = get_redis_conn(redis_db)?;
    auth_service::introduce_user_creation_demand(
        &mut r_conn,
        &pg_conn,
        mailer.as_ref(),
        model.get_email(),
    )?;
    info!("Got the request with the email {}", model.get_email());
    Ok(HttpResponse::Ok().finish())
}

/**
 * Function that is used to send token to users that forgot their password, these are then used to change their password.
 */
#[post("/forgotten/pwd")]
pub async fn request_pwd_change(
    redis_db: Data<RedisPool>,
    pg: Data<PgPool>,
    mailer: Data<Mailer>,
    model: Json<EmailHolder>,
) -> Result<HttpResponse, XamXamWebError> {
    let pg_conn: PgCon = get_pg_conn(pg)?;
    let mut r_conn: RCon = get_redis_conn(redis_db)?;
    auth_service::send_token_forgotten_pwd(&mut r_conn, &pg_conn, mailer.as_ref(), &model.0)?;
    info!(
        "A token to change password has been send to the user with email {} has been send.",
        model.get_email()
    );
    Ok(HttpResponse::Ok().finish())
}

#[post("/new/email")]
pub async fn request_new_email(
    redis_db: Data<RedisPool>,
    pg: Data<PgPool>,
    mailer: Data<Mailer>,
    model: Json<EmailHolder>,
    id: UserId,
) -> Result<HttpResponse, XamXamWebError> {
    info!("id {}", id.get_id());
    let pg_conn: PgCon = get_pg_conn(pg)?;
    let mut r_conn: RCon = get_redis_conn(redis_db)?;
    auth_service::request_email_change(
        &mut r_conn,
        &pg_conn,
        &model.0,
        id.get_id(),
        mailer.as_ref(),
    )?;
    Ok(HttpResponse::Ok().finish())
}
