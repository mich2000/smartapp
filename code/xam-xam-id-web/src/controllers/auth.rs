use crate::err::XamXamWebError;
use crate::{PgPool, RedisPool};
use crate::db::{get_pg_conn,get_redis_conn};
use actix_session::Session;
use actix_web::{HttpRequest,HttpResponse,get,post, web::Data, web::Json};
use actix_web_httpauth::headers::authorization::{Authorization,Basic};
use actix_web::http::header::Header;
use xam_xam_id_bll::viewmodels::new_user::NewUser;
use xam_xam_id_bll::auth_service;
use xam_xam_id_bll::{PgCon,R2D2Con};
use jwt_gang::claim_config::ClaimConfiguration;

/**
 * Function that is used to take in the a token and user creation info, the token will be controlled on the redis database. If the token is okay and the user doesn't already exist he is inserted in the database or otherwhise an error is returned.
 */
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
pub async fn login(req: HttpRequest,session : Session,pg : Data<PgPool>, jwt_config : Data<ClaimConfiguration>) -> Result<HttpResponse,XamXamWebError> {
    let pg_conn : PgCon = get_pg_conn(pg)?;
    let credentials = match Authorization::<Basic>::parse(&req) {
        Ok(credentials_scheme) => credentials_scheme,
        Err(e) => return Err(XamXamWebError::from(e))
    };
    let jwt_token = match auth_service::authenthicate_get_token(&pg_conn, jwt_config.as_ref(), credentials.as_ref().user_id(), credentials.as_ref().password().unwrap()) {
        Ok(token) => token,
        Err(e) => return Err(XamXamWebError::from(e))
    };
    session.set("Authorization", format!("Bearer {}",&jwt_token))?;
    info!("User with email {} has logged in",credentials.as_ref().user_id());
    Ok(HttpResponse::Ok().finish())
}

/**
 * Logs out a user by removing its token from the private cookies
 */
#[get("/logout")]
pub async fn logout(session : Session) -> HttpResponse {
    session.remove("Authorization");
    HttpResponse::Ok().finish()
}

/**
 * Function that is used to renew the token of a user. If the given token is not okay it will return an error.
 */
#[get("/renew/token")]
pub async fn renew_token(session : Session, jwt_config : Data<ClaimConfiguration>) ->  Result<HttpResponse,XamXamWebError> {
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
    let jwt_claim = jwt_config.as_ref().create_claim(&user_id_token.to_string())?;
    session.set("Authorization", format!("Bearer {}",jwt_config.as_ref().token_from_claim(&jwt_claim)?))?;
    Ok(HttpResponse::Ok().finish())
}