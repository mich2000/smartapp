use crate::schema::*;

#[derive(Debug,Associations,Queryable)]
#[belongs_to(crate::models::storage::Storage)]
#[table_name = "products"]
pub struct Product {
    pub id : i32,
    pub storage_id : i32,
    pub name : String,
    pub amount : usize,
    pub peremption_date: chrono::NaiveDate,
}