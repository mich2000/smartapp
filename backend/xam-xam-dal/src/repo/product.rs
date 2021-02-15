use crate::diesel::RunQueryDsl;
use crate::enums::product_kind::ProductKind;
use crate::err::XamXamError;
use crate::models::product::{Product, UpdateProduct};
use crate::models::product_id::ProductId;
use crate::repo::storage;
use crate::schema::products::dsl::*;
use crate::schema::products::*;
use crate::PgCon;
use chrono::NaiveDate;
use diesel::sql_types::{Date, Integer, Smallint, Text};
use diesel::ExpressionMethods;
use diesel::QueryDsl;

/**
 * Function used to insert a product into the products table in the database.
 */
pub fn insert_product(
    conn: &PgCon,
    user_id: i32,
    storage_name: &str,
    product_name: &str,
    product_amount: i16,
    product_peremption_date: NaiveDate,
    kind_of_product: ProductKind,
) -> Result<ProductId, XamXamError> {
    Ok(diesel::sql_query("INSERT INTO products(storage_id,name,amount,peremption_date,product_kind) values((select id from storages where name = $1 and user_id = $2),$3,$4,$5,$6::ProductKind) RETURNING id AS product_id")
    .bind::<Text,_>(storage_name)
    .bind::<Integer,_>(user_id)
    .bind::<Text,_>(product_name)
    .bind::<Smallint,_>(product_amount)
    .bind::<Date,_>(product_peremption_date)
    .bind::<Text,_>(kind_of_product.to_string())
    .get_result(conn)?)
}

pub fn move_product(
    conn: &PgCon,
    user_id: i32,
    product_id: i32,
    new_storage_name: &str,
) -> Result<(), XamXamError> {
    diesel::sql_query("update products p set storage_id = (select id from storages where user_id = $1 and name = $3) from storages s where s.user_id = $1 and p.id = $2 and s.id = p.storage_id")
    .bind::<Integer,_>(user_id)
    .bind::<Integer,_>(product_id)
    .bind::<Text,_>(new_storage_name)
    .execute(conn)?;
    Ok(())
}

pub fn update_product(
    conn: &PgCon,
    user_id: i32,
    storage_name: &str,
    model: &UpdateProduct,
) -> Result<(), XamXamError> {
    diesel::update(products.filter(storage_id.eq(storage::get_id_storage(
        conn,
        user_id,
        storage_name,
    )?)))
    .set(model)
    .execute(conn)?;
    Ok(())
}

pub fn get_products(
    conn: &PgCon,
    user_id: i32,
    storage_name: &str,
) -> Result<Vec<Product>, XamXamError> {
    Ok(
        diesel::sql_query("SELECT * FROM products WHERE storage_id = (SELECT id FROM storages WHERE name = $1 AND user_id = $2) order by peremption_date asc")
        .bind::<Text,_>(storage_name)
        .bind::<Integer,_>(user_id)
        .get_results::<Product>(conn)?
    )
}

pub fn delete_product(
    conn: &PgCon,
    user_id: i32,
    storage_name: &str,
    product_id: i32,
) -> Result<(), XamXamError> {
    diesel::sql_query("DELETE FROM products where storage_id = (select id from storages where name = $1 and user_id = $2) AND id = $3")
    .bind::<Text,_>(storage_name)
    .bind::<Integer,_>(user_id)
    .bind::<Integer,_>(product_id)
    .execute(conn)?;
    Ok(())
}
