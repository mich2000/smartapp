use crate::err::XamXamWebError;
use crate::{PgPool, RedisPool};
use crate::db::{get_pg_conn,get_redis_conn};
use actix_identity::Identity;
use actix_web::{HttpRequest,HttpResponse,get,post, web::Data, web::Json};
use actix_web_httpauth::headers::authorization::{Authorization,Basic};
use actix_web::http::header::Header;
use mailgang::mailer_gang::Mailer;
use xam_xam_id_bll::viewmodels::email::EmailHolder;
use xam_xam_id_bll::viewmodels::new_user::NewUser;
use xam_xam_id_bll::viewmodels::basic_info::UserInfo;
use xam_xam_id_bll::auth_service;
use xam_xam_id_bll::{PgCon,R2D2Con};
use jwt_gang::claim_config::ClaimConfiguration;
use crate::extractor::user_id::UserId;

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
#[get("/login")]
pub async fn login(req: HttpRequest,id : Identity,pg : Data<PgPool>, jwt_config : Data<ClaimConfiguration>) -> Result<HttpResponse,XamXamWebError> {
    let pg_conn : PgCon = get_pg_conn(pg)?;
    let credentials = match Authorization::<Basic>::parse(&req) {
        Ok(credentials_scheme) => credentials_scheme,
        Err(e) => {
            return Err(XamXamWebError::from(e))
        }
    };
    let jwt_token = match auth_service::authenthicate_get_token(&pg_conn, jwt_config.as_ref(), credentials.as_ref().user_id(), credentials.as_ref().password().unwrap()) {
        Ok(token) => token,
        Err(e) => {
            error!("{}",e);
            return Err(XamXamWebError::from(e))
        }
    };
    let user_id = jwt_config.as_ref().decode_token(&jwt_token)?.claims.get_subject().parse().unwrap();
    let basic_user_info = UserInfo::new(&match auth_service::get_basic_information(&pg_conn,user_id) {
        Ok(info) => info,
        Err(e) => {
            error!("{}",e);
            return Err(XamXamWebError::from(e))
        }
    });
    id.remember(format!("Bearer {}",&jwt_token));
    Ok(
        HttpResponse::Ok()
        .json(basic_user_info)
    )
}

#[get("/logout")]
pub async fn logout(id : Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().finish()
}

#[get("/basic/info")]
pub async fn get_basic_info(user_id : UserId,pg : Data<PgPool>) -> Result<HttpResponse,XamXamWebError> {
    let pg_conn : PgCon = get_pg_conn(pg)?;
    let user_id_token = match user_id.user_id().parse::<i32>() {
        Ok(id) => id,
        Err(_) => return Err(XamXamWebError::from("Could not parse string reference to i32"))
    };
    let basic_user_info = UserInfo::new(&match auth_service::get_basic_information(&pg_conn,user_id_token) {
        Ok(info) => info,
        Err(e) => {
            error!("{}",e);
            return Err(XamXamWebError::from(e))
        }
    });
    Ok(
        HttpResponse::Ok()
        .json(basic_user_info)
    )
}