use crate::db::GetCon;
use crate::err::XamXamWebError;
use crate::user_id::UserId;
use crate::PgPool;
use actix_web::{delete, get, post, put, web::Data, web::Json, HttpResponse};
use xam_xam_bis_bll::service::storage;
use xam_xam_bis_bll::viewmodels::edit_storage::EditStorage;
use xam_xam_bis_bll::viewmodels::new_storage::NewStorage;
use xam_xam_bis_bll::viewmodels::storage_name::StorageName;

#[get("/storages")]
pub async fn get_storages(id: UserId, pg: Data<PgPool>) -> Result<HttpResponse, XamXamWebError> {
    Ok(HttpResponse::Ok().json(storage::storage_list(&pg.conn()?, id.get_id())?))
}

#[get("/storages/names")]
pub async fn get_storages_storages(
    id: UserId,
    pg: Data<PgPool>,
) -> Result<HttpResponse, XamXamWebError> {
    Ok(HttpResponse::Ok().json(storage::storage_name_list(&pg.conn()?, id.get_id())?))
}

#[post("/storage")]
pub async fn add_storage(
    id: UserId,
    pg: Data<PgPool>,
    model: Json<NewStorage>,
) -> Result<HttpResponse, XamXamWebError> {
    storage::add_storage(&pg.conn()?, id.get_id(), &model.0)?;
    Ok(HttpResponse::Ok().finish())
}

#[put("/storage")]
pub async fn edit_storage(
    id: UserId,
    pg: Data<PgPool>,
    model: Json<EditStorage>,
) -> Result<HttpResponse, XamXamWebError> {
    storage::update_storage(&pg.conn()?, id.get_id(), &model.0)?;
    Ok(HttpResponse::Ok().finish())
}

#[delete("/storage")]
pub async fn delete_storage(
    id: UserId,
    pg: Data<PgPool>,
    model: Json<StorageName>,
) -> Result<HttpResponse, XamXamWebError> {
    storage::remove_storage(&pg.conn()?, id.get_id(), &model.0)?;
    Ok(HttpResponse::Ok().finish())
}
