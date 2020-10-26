use crate::err::XamXamWebError;
use crate::{PgPool, RedisPool};
use crate::db::{get_pg_conn,get_redis_conn};
use actix_identity::Identity;
use actix_web::{HttpResponse,post, web::Data, web::Json};
use mailgang::mailer_gang::Mailer;
use xam_xam_id_bll::viewmodels::email::EmailHolder;
use xam_xam_id_bll::viewmodels::new_user::NewUser;
use xam_xam_id_bll::viewmodels::email_pwd::EmailAndPwdHolder;
use xam_xam_id_bll::viewmodels::basic_info::UserInfo;
use xam_xam_id_bll::auth_service;
use xam_xam_id_bll::{PgCon,R2D2Con};
use jwt_gang::claim_config::ClaimConfiguration;

/**
 * Route that is used to request a token received on the email, this token is then used to make a new user.
 */
#[post("/new/user")]
pub async fn request_new_user(redis_db : Data<RedisPool>, pg : Data<PgPool>, mailer : Data<Mailer>, model: Json<EmailHolder>) -> Result<HttpResponse,XamXamWebError> {
    let pg_conn : PgCon = get_pg_conn(pg)?;
    let mut r_conn : R2D2Con = get_redis_conn(redis_db)?;
    auth_service::introduce_user_creation_demand(&mut r_conn, &pg_conn, mailer.as_ref(), model.get_email())?;
    info!("Got the request with the email {}",model.get_email());
    Ok(HttpResponse::Ok().finish())
}

#[post("/register")]
pub async fn register(redis_db : Data<RedisPool>, pg : Data<PgPool>, model: Json<NewUser>) -> Result<HttpResponse,XamXamWebError> {
    let pg_conn : PgCon = get_pg_conn(pg)?;
    let mut r_conn : R2D2Con = get_redis_conn(redis_db)?;
    auth_service::create_user(&mut r_conn, &pg_conn, &model)?;
    info!("A new user with the email {} has been made", model.get_email());
    Ok(HttpResponse::Ok().finish())
}

/**
 * Function that controls the identity of someone by controlling its email and password. In return it will give the user a private cookie with a jwt token in it. And it will return a json in it will give you basic user information.
 */
#[post("/login")]
pub async fn login(id : Identity,pg : Data<PgPool>, jwt_config : Data<ClaimConfiguration>, model : Json<EmailAndPwdHolder>) -> Result<HttpResponse,XamXamWebError> {
    let pg_conn : PgCon = get_pg_conn(pg)?;
    let jwt_token = auth_service::authenthicate_get_token(&pg_conn, jwt_config.as_ref(), model.get_email(), model.get_pwd())?;
    let user_id = jwt_config.as_ref().decode_token(&jwt_token)?.claims.get_subject().parse().unwrap();
    let basic_user_info = UserInfo::new(&auth_service::get_basic_information(&pg_conn,user_id)?);
    id.remember(format!("Bearer {}",jwt_token));
    Ok(
        HttpResponse::Ok()
        .json(basic_user_info)
    )
}

#[post("/logout")]
pub async fn logout(id : Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().finish()
}