use crate::err::XamXamWebError;
use crate::db::{get_pg_conn,get_redis_conn};
use actix_web::{HttpResponse,get,web::Data,web::Json,put,delete};
use xam_xam_id_bll::viewmodels::basic_info::UserInfo;
use xam_xam_id_bll::viewmodels::new_email::NewEmailHolder;
use xam_xam_id_bll::viewmodels::password::PasswordHolder;
use xam_xam_id_bll::auth_service;
use xam_xam_id_bll::{PgCon,RCon};
use crate::extractors::{user_id::UserId,credentials::Cred};
use crate::{PgPool, RedisPool};
use actix_identity::Identity;

#[get("/basic/info")]
pub async fn get_basic_info(id : UserId,pg : Data<PgPool>) -> Result<HttpResponse,XamXamWebError> {
    let pg_conn : PgCon = get_pg_conn(pg)?;
    let basic_user_info = UserInfo::new(&match auth_service::get_basic_information(&pg_conn,id.get_id()) {
        Ok(info) => info,
        Err(e) => return Err(XamXamWebError::from(e))
    });
    Ok(
        HttpResponse::Ok()
        .json(basic_user_info)
    )
}

#[put("/change/email")]
pub async fn change_email(redis_db : Data<RedisPool>, pg : Data<PgPool>, model: Json<NewEmailHolder>,id : UserId) -> Result<HttpResponse,XamXamWebError> {
    let pg_conn : PgCon = get_pg_conn(pg)?;
    let mut r_conn : RCon = get_redis_conn(redis_db)?;
    auth_service::change_email(&mut r_conn, &pg_conn, id.get_id(), &model.0)?;
    Ok(HttpResponse::Ok().finish())
}

#[put("/change/password")]
pub async fn change_password(credentials : Cred,pg : Data<PgPool>, model: Json<PasswordHolder>) -> Result<HttpResponse,XamXamWebError> {
    let pg_conn : PgCon = get_pg_conn(pg)?;
    auth_service::change_pwd(&pg_conn, &model.0,&credentials.get_name(),&credentials.get_password())?;
    Ok(HttpResponse::Ok().finish())
}

#[delete("/delete/profile")]
pub async fn delete_profile(credentials : Cred,pg : Data<PgPool>, session : Identity) -> Result<HttpResponse,XamXamWebError> {
    let pg_conn : PgCon = get_pg_conn(pg)?;
    auth_service::delete_user(&pg_conn,&credentials.get_name(),&credentials.get_password())?;
    session.forget();
    Ok(HttpResponse::Ok().finish())
}