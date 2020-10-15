use crate::enums::storage_kind::StorageKind;
use crate::models::user::User;
use crate::err::XamXamError;
use crate::schema::*;

/**
 * Struct that represents the Storage unit where products lie.
 */
#[derive(Insertable, Queryable, Associations, Identifiable, Debug, PartialEq, Clone)]
#[belongs_to(User)]
#[table_name = "storages"]
pub struct Storage {
    pub id : i32,
    pub user_id : i32,
    pub name : String,
    pub storage_kind : StorageKind
}

/**
 * Struct that is used to insert a storage in the database.
 */
#[derive(Insertable,Debug)]
#[table_name = "storages"]
pub struct InsertableStorage {
    pub user_id : i32,
    pub name : String,
    pub storage_kind : StorageKind
}

impl InsertableStorage {
    /**
     * Creates a new InsertableStorage object where the storage kind is optional. If it the storage kind is empty, it will be set as a default storage kind, which is the other value.
     */
    pub fn new(user_id : i32, storage_name : &str, storage_kind : Option<StorageKind>) -> Result<InsertableStorage,XamXamError> {
        if storage_name.is_empty() {
            return Err(XamXamError::StorageNameIsEmpty)
        }
        Ok(
            Self {
                user_id : user_id,
                name : storage_name.to_string(),
                storage_kind : storage_kind.unwrap_or(StorageKind::Other)
            }
        )
    }
}