use crate::err::XamXamError;
use crate::models::product::Product;
use crate::PgCon;
use chrono::NaiveDate;
use crate::enums::product_kind::ProductKind;
use crate::diesel::RunQueryDsl;
use diesel::sql_types::{Text, Date, Integer};

/**
 * Function used to insert a product into the products table in the database.
 */
pub fn insert_product(conn: &PgCon, product_user_id : i32, storage_name : &str, product_name : &str, product_amount : i32, product_peremption_date : NaiveDate, kind_of_product : ProductKind) -> Result<(), XamXamError> {
    diesel::sql_query("INSERT INTO products(storage_id,name,amount,peremption_date,product_kind) values((select id from storages where name = $1 and user_id = $2),$3,$4,$5,$6)")
    .bind::<Text,_>(storage_name)
    .bind::<Integer,_>(product_user_id)
    .bind::<Text,_>(product_name)
    .bind::<Integer,_>(product_amount)
    .bind::<Date,_>(product_peremption_date)
    .bind::<Text,_>(kind_of_product.to_string())
    .execute(conn)?;
    Ok(())
}

pub fn get_products(conn: &PgCon, product_user_id : i32, storage_name : &str) -> Result<Vec<Product>,XamXamError> {
    Ok(
        diesel::sql_query("SELECT * FROM products WHERE storage_id = (SELECT id FROM storages WHERE name = $1 AND user_id = $2)")
        .bind::<Text,_>(storage_name)
        .bind::<Integer,_>(product_user_id)
        .get_results::<Product>(conn)?
    )
}

pub fn delete_product(conn: &PgCon, product_user_id : i32, storage_name : &str, product_id : i32) -> Result<(), XamXamError> {
    diesel::sql_query("DELETE FROM products where storage_id = (select id from storages where name = $1 and user_id = $2) AND id = $3")
    .bind::<Text,_>(storage_name)
    .bind::<Integer,_>(product_user_id)
    .bind::<Integer,_>(product_id)
    .execute(conn)?;
    Ok(())
}