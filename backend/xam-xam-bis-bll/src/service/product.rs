use crate::viewmodels::insert_product::InsertProduct;
use crate::viewmodels::id_product::IdProduct;
use crate::viewmodels::product_id::ProductId;
use crate::viewmodels::products::Products;
use crate::err::XamXamServiceError;
use crate::PgCon;
use xam_xam_dal::repo::product;

pub fn add_product(conn : &PgCon, user_id : i32, model : &InsertProduct) -> Result<ProductId,XamXamServiceError> {
    if model.get_storage_name().is_empty() {
        return Err(XamXamServiceError::StorageNameIsEmpty)
    }
    Ok(ProductId::new(
        product::insert_product(conn, user_id, model.get_storage_name(), model.get_name(), model.get_amount(), model.get_peremption_date().to_owned(), model.get_kind())?.get_id()
    ))
}

pub fn get_product_list(conn : &PgCon, user_id : i32, storage_name : &str) -> Result<Products,XamXamServiceError> {
    if storage_name.is_empty() {
        return Err(XamXamServiceError::StorageNameIsEmpty)
    }
    Ok(product::get_products(conn, user_id, storage_name)?.into())
}

pub fn remove_product(conn : &PgCon, user_id : i32, model : &IdProduct) -> Result<(),XamXamServiceError> {
    if model.get_storage_name().is_empty() {
        return Err(XamXamServiceError::StorageNameIsEmpty)
    }
    product::delete_product(conn, user_id, model.get_storage_name(), model.get_id())?;
    Ok(())
}