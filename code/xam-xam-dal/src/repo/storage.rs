use crate::diesel::query_dsl::filter_dsl::FilterDsl;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::err::XamXamError;
use crate::models::storage::{InsertableStorage, Storage, UpdateStorage};
use crate::schema::storages::dsl::*;
use crate::schema::storages::*;
use crate::PgCon;

/**
 * Inserts a storage in a database
 */
pub fn insert_storage(conn: &PgCon, storage: &InsertableStorage) -> Result<(), XamXamError> {
    match diesel::insert_into(storages)
        .values(storage)
        .get_result::<Storage>(conn)
    {
        Ok(inserted_storage) => {
            info!(
                "The storage {} has been added to the user with id {}",
                &inserted_storage.name, &inserted_storage.user_id
            );
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

/**
 * Function that is used to return all storages from a specific user.
 */
pub fn get_storages(conn: &PgCon, id_user: i32) -> Result<Vec<Storage>, XamXamError> {
    match storages
        .filter(user_id.eq(id_user))
        .get_results::<Storage>(conn)
    {
        Ok(storages_vector) => Ok(storages_vector),
        Err(e) => Err(e.into()),
    }
}

/**
 * Update the storage by looking up its name and user id and then applies a struct that contains the information to update the storage. Function can returns a true if it affected a storage, false if no things were changed.
 */
pub fn update_storage(
    conn: &PgCon,
    id_user: i32,
    storage_name: &str,
    update_model: &UpdateStorage,
) -> Result<bool, XamXamError> {
    if storage_name.is_empty() {
        return Err(XamXamError::StorageNameIsEmpty);
    }
    if update_model.name.is_none() && update_model.storage_kind.is_none() {
        info!(
            "Storage {} of user {} could not be updated, because the update model was empty",
            storage_name, id_user
        );
        return Ok(false);
    }
    match diesel::update(
        storages
            .filter(user_id.eq(id_user))
            .filter(name.eq(storage_name)),
    )
    .set(update_model)
    .execute(conn)
    {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                info!(
                    "Storage {} of user {} has been updated",
                    storage_name, id_user
                );
                Ok(true)
            } else {
                info!(
                    "Storage {} of user {} could not be updated",
                    storage_name, id_user
                );
                Ok(false)
            }
        }
        Err(e) => Err(e.into()),
    }
}

/**
 * Function that is used to delete a storage based on it user_id and name. A boolean will be given back, a true means a storage has been deleted and a false means nothing happened.
 */
pub fn delete_storage(conn: &PgCon, id_user: i32, storage_name: &str) -> Result<bool, XamXamError> {
    if storage_name.is_empty() {
        return Err(XamXamError::StorageNameIsEmpty);
    }
    match diesel::delete(
        storages
            .filter(user_id.eq(id_user))
            .filter(name.eq(storage_name)),
    )
    .execute(conn)
    {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                info!(
                    "Storage {} of user {} has been deleted",
                    storage_name, id_user
                );
                Ok(true)
            } else {
                info!(
                    "Storage {} of user {} could not be deleted",
                    storage_name, id_user
                );
                Ok(false)
            }
        }
        Err(e) => Err(e.into()),
    }
}
