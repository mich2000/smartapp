use crate::err::XamXamWebError;
use crate::PgPool;
use crate::db::get_pg_conn;
use actix_web::{HttpResponse,get,web::Data};
use xam_xam_id_bll::viewmodels::basic_info::UserInfo;
use xam_xam_id_bll::auth_service;
use xam_xam_id_bll::PgCon;
use crate::extractors::user_id::UserId;

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