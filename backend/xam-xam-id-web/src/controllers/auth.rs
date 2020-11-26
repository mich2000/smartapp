use crate::err::XamXamWebError;
use crate::extractors::credentials::Cred;
use crate::{PgPool, RedisPool};
use actix_identity::Identity;
use actix_web::{get, post, put, web::Data, web::Json, HttpResponse};
use actix_web_httpauth::headers::authorization::Bearer;
use jwt_gang::claim_config::ClaimConfiguration;
use xam_xam_id_bll::auth_service;
use xam_xam_id_bll::viewmodels::forgot_pwd::ForgottenPassword;
use xam_xam_id_bll::viewmodels::email::EmailHolder;
use xam_xam_id_bll::viewmodels::new_user::NewUser;
use xam_xam_id_bll::RCon;
use crate::db::GetCon;

/**
 * Function that is used to take in the a token and user creation info, the token will be controlled on the redis database. If the token is okay and the user doesn't already exist he is inserted in the database or otherwhise an error is returned.
 */
#[post("/register")]
pub async fn register(
    redis_db: Data<RedisPool>,
    pg: Data<PgPool>,
    model: Json<NewUser>,
) -> Result<HttpResponse, XamXamWebError> {
    let mut r_conn: RCon = redis_db.conn()?;
    auth_service::create_user(&mut r_conn, &pg.conn()?, &model)?;
    info!(
        "A new user with the email {} has been made",
        model.get_email()
    );
    Ok(HttpResponse::Ok().finish())
}

/**
 * Function that controls the identity of someone by controlling its email and password. In return it will give the user a private cookie with a jwt token in it. And it will return a json in it will give you basic user information.
 */
#[get("/login")]
pub async fn login(
    credentials: Cred,
    session: Identity,
    pg: Data<PgPool>,
    jwt_config: Data<ClaimConfiguration>,
) -> Result<HttpResponse, XamXamWebError> {
    let jwt_token: String = match auth_service::authenthicate_get_token(
        &pg.conn()?,
        jwt_config.as_ref(),
        &credentials.get_name(),
        &credentials.get_password(),
    ) {
        Ok(token) => token,
        Err(e) => return Err(XamXamWebError::from(e)),
    };
    session.remember(
        Bearer::new(jwt_token)
            .token()
            .to_owned()
            .as_ref()
            .to_string(),
    );
    Ok(HttpResponse::Ok().finish())
}

/**
 * Logs out a user by removing its token from the private cookies
 */
#[get("/logout")]
pub async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().finish()
}

#[get("/validate")]
pub async fn validate(
    session: Identity,
    jwt_config: Data<ClaimConfiguration>,
) -> Result<HttpResponse, XamXamWebError> {
    let jwt_token = match session.identity() {
        Some(token) => token,
        None => return Err(XamXamWebError::CredentialsNotPresent),
    };
    jwt_config.decode_token(&jwt_token)?;
    Ok(HttpResponse::Ok().finish())
}

/**
 * Function that is used to renew the token of a user. If the given token is not okay it will return an error.
 */
#[get("/renew/token")]
pub async fn renew_token(
    session: Identity,
    jwt_config: Data<ClaimConfiguration>,
) -> Result<HttpResponse, XamXamWebError> {
    let jwt_token = match session.identity() {
        Some(token) => token,
        None => return Err(XamXamWebError::CredentialsNotPresent),
    };
    let token_subject = &jwt_config
        .decode_token(&jwt_token)?
        .claims
        .get_subject()
        .to_owned();
    let user_id_token = match token_subject.parse::<i32>() {
        Ok(id) => id,
        Err(_) => return Err(XamXamWebError::from(
            "Could not parse string reference to i32",
        ))
    };
    let jwt_claim = jwt_config
        .as_ref()
        .create_claim(&user_id_token.to_string())?;
    let token = jwt_config.token_from_claim(&jwt_claim)?;
    let new_token = Bearer::new(token).token().to_owned().as_ref().to_string();
    session.remember(new_token);
    Ok(HttpResponse::Ok().finish())
}

/**
 * Function that is used to change the password of a user has forgotten, to change the password the user must exist and give the right token taht will be checked in the redis database.
*/
#[put("/change/forgotten/pwd")]
pub async fn change_forgotten_pwd(
    redis_db: Data<RedisPool>,
    pg: Data<PgPool>,
    model: Json<ForgottenPassword>,
) -> Result<HttpResponse, XamXamWebError> {
    let mut r_conn: RCon = redis_db.conn()?;
    auth_service::change_forgotten_pwd(&mut r_conn, &pg.conn()?, &model)?;
    Ok(HttpResponse::Ok().finish())
}

#[get("/email")]
pub async fn get_email(
    session: Identity,
    pg: Data<PgPool>,
    jwt_config: Data<ClaimConfiguration>
) -> Result<HttpResponse, XamXamWebError> {
    let jwt_token = match session.identity() {
        Some(token) => token,
        None => return Err(XamXamWebError::CredentialsNotPresent),
    };
    let token_subject = &jwt_config
        .decode_token(&jwt_token)?
        .claims
        .get_subject()
        .to_owned();
    let user_id_token = match token_subject.parse::<i32>() {
        Ok(id) => id,
        Err(_) => return Err(XamXamWebError::from(
            "Could not parse string reference to i32",
        ))
    };
    Ok(HttpResponse::Ok().json::<EmailHolder>(auth_service::get_email_from_id(user_id_token, &pg.conn()?)?))
}
