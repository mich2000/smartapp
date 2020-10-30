use crate::err::XamXamWebError;
use crate::PgPool;
use crate::db::get_pg_conn;
use actix_session::Session;
use actix_web::{HttpResponse,get,web::Data};
use xam_xam_id_bll::viewmodels::basic_info::UserInfo;
use xam_xam_id_bll::auth_service;
use xam_xam_id_bll::PgCon;
use jwt_gang::claim_config::ClaimConfiguration;

#[get("/basic/info")]
pub async fn get_basic_info(session : Session,pg : Data<PgPool>, jwt_config : Data<ClaimConfiguration>) -> Result<HttpResponse,XamXamWebError> {
    let pg_conn : PgCon = get_pg_conn(pg)?;
    let user_id = match session.get::<String>("Authorization")? {
        Some(token) => token,
        None => return Err(XamXamWebError::CredentialsNotPresent)
    };
    let split : Vec<&str> = user_id.split("Bearer").collect();
    let user_id_from_split = jwt_config.as_ref().decode_token(&split[1].trim())?.claims.get_subject().to_owned();
    let user_id_token = match user_id_from_split.parse::<i32>() {
        Ok(id) => id,
        Err(_) => return Err(XamXamWebError::from("Could not parse string reference to i32"))
    };
    let basic_user_info = UserInfo::new(&match auth_service::get_basic_information(&pg_conn,user_id_token) {
        Ok(info) => info,
        Err(e) => return Err(XamXamWebError::from(e))
    });
    Ok(
        HttpResponse::Ok()
        .json(basic_user_info)
    )
}