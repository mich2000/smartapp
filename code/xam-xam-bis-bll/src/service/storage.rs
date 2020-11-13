use crate::err::XamXamServiceError;
use crate::viewmodels::new_storage::NewStorage;
use crate::viewmodels::storage_name::StorageName;
use crate::viewmodels::storages::Storages;
use crate::PgCon;
use xam_xam_dal::models::storage::{InsertableStorage, Storage, UpdateStorage};
use xam_xam_dal::repo::storage;

pub fn storage_list(
    conn: &PgCon,
    user_id: i32
) -> Result<Storages,XamXamServiceError> {
    Ok(storage::get_storages(conn, user_id)?.into())
}

/**
 * Adds a storage to an user associated with its user id.
*/
pub fn add_storage(
    conn: &PgCon,
    user_id: i32,
    model: &NewStorage,
) -> Result<(), XamXamServiceError> {
    error!("{:?}",storage::insert_storage(
        conn,
        &InsertableStorage::new(user_id, model.get_name(), model.get_kind())?,
    )?);
    info!("A new storage has been added.");
    Ok(())
}

pub fn remove_storage(
    conn: &PgCon,
    user_id: i32,
    model : &StorageName
) -> Result<(), XamXamServiceError> {
    storage::delete_storage(conn, user_id, model.get_name())?;
    info!("A storage has been deleted.");
    Ok(())
}
