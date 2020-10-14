use crate::schema::*;
use crate::enums::product_kind::ProductKind;

#[derive(Queryable, Associations, Identifiable, Debug, PartialEq)]
#[belongs_to(crate::models::storage::Storage)]
#[table_name = "products"]
pub struct Product {
    pub id : i32,
    pub storage_id : i32,
    pub name : String,
    pub amount : usize,
    pub peremption_date: chrono::NaiveDate,
    pub product_kind : ProductKind
}