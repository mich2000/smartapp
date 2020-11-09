use crate::diesel::query_dsl::filter_dsl::FilterDsl;
use crate::diesel::query_dsl::filter_dsl::FindDsl;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::err::XamXamError;
use crate::models::product::{InsertableProduct, Product, UpdateProduct};
use crate::schema::products::dsl::*;
use crate::schema::products::*;
use crate::PgCon;

/**
 * Function used to insert a product into the products table in the database.
 */
pub fn insert_product(conn: &PgCon, product: &InsertableProduct) -> Result<(), XamXamError> {
    match diesel::insert_into(products)
        .values(product)
        .get_result::<Product>(conn)
    {
        Ok(inserted_product) => {
            info!(
                "A product {} has been inserted in a storage id {}.",
                &inserted_product.name, &inserted_product.storage_id
            );
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

/**
 * Function that returns a vector of products through the storage id.
 */
pub fn get_products(conn: &PgCon, id_storage: i32) -> Result<Vec<Product>, XamXamError> {
    match products
        .filter(storage_id.eq(id_storage))
        .get_results::<Product>(conn)
    {
        Ok(products_vector) => Ok(products_vector),
        Err(e) => Err(e.into()),
    }
}

/**
 * Function that is used to update a product based on its id storage and product id. Will return a true result if the product has been updated and false if nothing was updated.
 */
pub fn update_product(
    conn: &PgCon,
    id_storage: i32,
    id_product: i32,
    update_model: &UpdateProduct,
) -> Result<bool, XamXamError> {
    match diesel::update(products.filter(storage_id.eq(id_storage)).find(id_product))
        .set(update_model)
        .execute(conn)
    {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                info!(
                    "product {} has succesfully been updated, its storage was {}",
                    id_product, id_storage
                );
                Ok(true)
            } else {
                info!(
                    "product {} could not be updated, its storage was {}",
                    id_product, id_storage
                );
                Ok(false)
            }
        }
        Err(e) => Err(e.into()),
    }
}

/**
 * Function that is used to delete a product based on it storage id where it belongs and its product id. A true result is returned when a product has been deleted, otherwhise a false is given.
 */
pub fn delete_product(conn: &PgCon, id_storage: i32, id_product: i32) -> Result<bool, XamXamError> {
    match diesel::delete(products.filter(storage_id.eq(id_storage)).find(id_product)).execute(conn)
    {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                info!(
                    "product {} has succesfully been deleted, its storage was {}",
                    id_product, id_storage
                );
                Ok(true)
            } else {
                info!(
                    "product {} could not be deleted, its storage was {}",
                    id_product, id_storage
                );
                Ok(false)
            }
        }
        Err(e) => Err(e.into()),
    }
}
