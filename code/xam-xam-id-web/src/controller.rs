use actix_web::{get, post, web, web::Data, web::Json};
use mailgang::mailer_gang::Mailer;
use crate::err::XamXamWebError;
use crate::{PgPool, RedisPool};
use xam_xam_id_bll::viewmodels::email::EmailHolder;
use xam_xam_id_bll::auth_service;

#[post("/request/new/user")]
pub async fn request_new_user(redis_db : Data<RedisPool>, pg : Data<PgPool>, mailer : Data<Mailer>, model: Json<EmailHolder>) -> Result<String,XamXamWebError> {
    let mut r_conn = redis_db.get().unwrap();
    let p_conn = pg.get().unwrap();
    info!("Got the request with the email {}",model.get_email());
    auth_service::introduce_user_creation_demand(&mut r_conn, &p_conn, mailer.get_ref(), model.get_email())?;
    Ok("Email with token has been sent".to_string())
}