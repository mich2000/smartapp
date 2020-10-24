use actix_web::{post, web::Data, web::Json};
use mailgang::mailer_gang::Mailer;
use crate::err::XamXamWebError;
use crate::{PgPool, RedisPool};
use xam_xam_id_bll::viewmodels::email::EmailHolder;
use xam_xam_id_bll::auth_service;
use actix_web::HttpResponse;

#[post("/request/new/user")]
pub async fn request_new_user(redis_db : Data<RedisPool>, pg : Data<PgPool>, mailer : Data<Mailer>, model: Json<EmailHolder>) -> Result<HttpResponse,XamXamWebError> {
    let mut r_conn = redis_db.get().unwrap();
    let p_conn = pg.get().unwrap();
    info!("Got the request with the email {}",model.get_email());
    auth_service::introduce_user_creation_demand(&mut r_conn, &p_conn, mailer.as_ref(), model.get_email())?;
    Ok(HttpResponse::Ok().finish())
}