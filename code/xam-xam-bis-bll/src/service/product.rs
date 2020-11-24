use crate::viewmodels::insert_product::InsertProduct;
use crate::viewmodels::storage_name::StorageName;
use crate::viewmodels::id_product::IdProduct;
use crate::viewmodels::products::Products;
use crate::err::XamXamServiceError;
use crate::PgCon;
use xam_xam_dal::repo::product;

pub fn add_product(conn : &PgCon, user_id : i32, model : &InsertProduct) -> Result<(),XamXamServiceError> {
    if model.get_storage_name().is_empty() {
        return Err(XamXamServiceError::StorageNameIsEmpty)
    }
    product::insert_product(conn, user_id, model.get_storage_name(), model.get_name(), model.get_amount(), model.get_peremption_date().to_owned(), model.get_kind())?;
    Ok(())
}

pub fn get_product_list(conn : &PgCon, user_id : i32, model : &StorageName) -> Result<Products,XamXamServiceError> {
    if model.get_name().is_empty() {
        return Err(XamXamServiceError::StorageNameIsEmpty)
    }
    Ok(product::get_products(conn, user_id, model.get_name())?.into())
}

pub fn remove_product(conn : &PgCon, user_id : i32, model : &IdProduct) -> Result<(),XamXamServiceError> {
    if model.get_storage_name().is_empty() {
        return Err(XamXamServiceError::StorageNameIsEmpty)
    }
    product::delete_product(conn, user_id, model.get_storage_name(), model.get_id())?;
    Ok(())
}