use crate::db::get_pg_conn;
use crate::err::XamXamWebError;
use crate::user_id::UserId;
use crate::PgPool;
use actix_web::{get, post, delete, web::Data, web::Json, HttpResponse};
use xam_xam_bis_bll::viewmodels::new_storage::NewStorage;
use xam_xam_bis_bll::viewmodels::storage_name::StorageName;
use xam_xam_bis_bll::service::storage;

#[get("/storages")]
pub async fn get_storages(id : UserId, pg : Data<PgPool>) -> Result<HttpResponse,XamXamWebError> {
    let pg = get_pg_conn(pg)?;
    Ok(HttpResponse::Ok().json(storage::storage_list(&pg, id.get_id())?))
}

#[post("/storage")]
pub async fn add_storage(id : UserId, pg : Data<PgPool>, model : Json<NewStorage>) -> Result<HttpResponse,XamXamWebError> {
    let pg = get_pg_conn(pg)?;
    storage::add_storage(&pg, id.get_id(), &model.0)?;
    Ok(HttpResponse::Ok().finish())
}

#[delete("/storage")]
pub async fn delete_storage(id : UserId, pg : Data<PgPool>, model : Json<StorageName>) -> Result<HttpResponse,XamXamWebError> {
    let pg = get_pg_conn(pg)?;
    storage::remove_storage(&pg, id.get_id(), &model.0)?;
    Ok(HttpResponse::Ok().finish())
}