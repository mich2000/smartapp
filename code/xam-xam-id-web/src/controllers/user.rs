use crate::err::XamXamWebError;
use crate::db::{get_pg_conn,get_redis_conn};
use actix_web::{HttpResponse,get,web::Data,web::Json,post};
use xam_xam_id_bll::viewmodels::basic_info::UserInfo;
use xam_xam_id_bll::viewmodels::new_email::NewEmailHolder;
use xam_xam_id_bll::auth_service;
use xam_xam_id_bll::{PgCon,R2D2Con};
use crate::extractors::user_id::UserId;
use crate::{PgPool, RedisPool};

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

#[post("/change/email")]
pub async fn change_email(redis_db : Data<RedisPool>, pg : Data<PgPool>, model: Json<NewEmailHolder>,id : UserId) -> Result<HttpResponse,XamXamWebError> {
    let pg_conn : PgCon = get_pg_conn(pg)?;
    let mut r_conn : R2D2Con = get_redis_conn(redis_db)?;
    auth_service::change_email(&mut r_conn, &pg_conn, id.get_id(), &model.0)?;
    Ok(HttpResponse::Ok().finish())
}