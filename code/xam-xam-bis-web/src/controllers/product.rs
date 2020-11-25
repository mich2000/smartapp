use xam_xam_bis_bll::viewmodels::insert_product::InsertProduct;
use xam_xam_bis_bll::viewmodels::storage_name::StorageName;
use xam_xam_bis_bll::viewmodels::id_product::IdProduct;
use xam_xam_bis_bll::service::product;
use crate::db::GetCon;
use crate::err::XamXamWebError;
use crate::user_id::UserId;
use crate::PgPool;
use actix_web::{get, post, delete, web::Data, web::Json, HttpResponse};

#[get("/products")]
pub async fn get_product_list(id : UserId, pg : Data<PgPool>, model : Json<StorageName>) -> Result<HttpResponse,XamXamWebError> {
    Ok(HttpResponse::Ok().json(product::get_product_list(&pg.conn()?, id.get_id(), &model.0)?))
}

#[post("/product")]
pub async fn add_product(id : UserId, pg : Data<PgPool>, model : Json<InsertProduct>) -> Result<HttpResponse,XamXamWebError> {
    product::add_product(&pg.conn()?, id.get_id(), &model.0)?;
    Ok(HttpResponse::Ok().finish())
}

#[delete("/product")]
pub async fn remove_product(id : UserId, pg : Data<PgPool>, model : Json<IdProduct>) -> Result<HttpResponse,XamXamWebError> {
    product::remove_product(&pg.conn()?, id.get_id(), &model.0)?;
    Ok(HttpResponse::Ok().finish())
}