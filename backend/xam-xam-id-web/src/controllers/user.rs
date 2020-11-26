use crate::db::GetCon;
use crate::err::XamXamWebError;
use crate::extractors::{credentials::Cred, user_id::UserId};
use crate::{PgPool, RedisPool};
use actix_identity::Identity;
use actix_web::{delete, get, put, web::Data, web::Json, HttpResponse};
use xam_xam_id_bll::auth_service;
use xam_xam_id_bll::viewmodels::basic_info::UserInfo;
use xam_xam_id_bll::viewmodels::new_email::NewEmailHolder;
use xam_xam_id_bll::viewmodels::password::PasswordHolder;
use xam_xam_id_bll::RCon;

#[get("/basic/info")]
pub async fn get_basic_info(id: UserId, pg: Data<PgPool>) -> Result<HttpResponse, XamXamWebError> {
    let basic_user_info = UserInfo::new(&match auth_service::get_basic_information(
        &pg.conn()?,
        id.get_id(),
    ) {
        Ok(info) => info,
        Err(e) => return Err(XamXamWebError::from(e)),
    });
    Ok(HttpResponse::Ok().json(basic_user_info))
}

#[put("/change/email")]
pub async fn change_email(
    redis_db: Data<RedisPool>,
    pg: Data<PgPool>,
    model: Json<NewEmailHolder>,
    id: UserId,
) -> Result<HttpResponse, XamXamWebError> {
    let mut r_conn: RCon = redis_db.conn()?;
    auth_service::change_email(&mut r_conn, &pg.conn()?, id.get_id(), &model.0)?;
    Ok(HttpResponse::Ok().finish())
}

#[put("/change/password")]
pub async fn change_password(
    credentials: Cred,
    pg: Data<PgPool>,
    model: Json<PasswordHolder>,
) -> Result<HttpResponse, XamXamWebError> {
    auth_service::change_pwd(
        &pg.conn()?,
        &model.0,
        &credentials.get_name(),
        &credentials.get_password(),
    )?;
    Ok(HttpResponse::Ok().finish())
}

#[delete("/delete/profile")]
pub async fn delete_profile(
    credentials: Cred,
    pg: Data<PgPool>,
    session: Identity,
) -> Result<HttpResponse, XamXamWebError> {
    auth_service::delete_user(
        &pg.conn()?,
        &credentials.get_name(),
        &credentials.get_password(),
    )?;
    session.forget();
    Ok(HttpResponse::Ok().finish())
}
