use crate::enums::storage_kind::StorageKind;
use crate::models::user::User;
use crate::schema::*;

#[derive(Debug,Associations,Queryable)]
#[belongs_to(User)]
#[table_name = "storages"]
pub struct Storage {
    pub id : i32,
    pub user_id : i32,
    pub name : String,
    pub storage_kind : StorageKind
}