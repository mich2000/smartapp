use crate::err::XamXamError;
use crate::models::storage::{InsertableStorage, Storage, UpdateStorage};
use crate::schema::storages::dsl::*;
use crate::schema::storages::*;
use crate::PgCon;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

/**
 * Inserts a storage in a database
 */
pub fn insert_storage(conn: &PgCon, storage: &InsertableStorage) -> Result<(), XamXamError> {
    diesel::insert_into(storages)
        .values(storage)
        .execute(conn)?;
    Ok(())
}

/**
 * Function that is used to return all storages from a specific user.
 */
pub fn get_storages(conn: &PgCon, id_user: i32) -> Result<Vec<Storage>, XamXamError> {
    Ok(storages
        .filter(user_id.eq(id_user))
        .get_results::<Storage>(conn)?)
}

/**
 * Method to get all storage names of a particular user by his user id.
 */
pub fn get_storage_names(conn: &PgCon, id_user: i32) -> Result<Vec<String>, XamXamError> {
    Ok(
        storages
        .filter(user_id.eq(id_user))
        .select(name)
        .get_results::<String>(conn)?
    )
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
    Ok(diesel::update(
        storages
            .filter(user_id.eq(id_user))
            .filter(name.eq(storage_name)),
    )
    .set(update_model)
    .execute(conn)?
        > 0)
}

/**
 * Function that is used to delete a storage based on it user_id and name. A boolean will be given back, a true means a storage has been deleted and a false means nothing happened.
 */
pub fn delete_storage(conn: &PgCon, id_user: i32, storage_name: &str) -> Result<bool, XamXamError> {
    if storage_name.is_empty() {
        return Err(XamXamError::StorageNameIsEmpty);
    }
    Ok(diesel::delete(
        storages
            .filter(user_id.eq(id_user))
            .filter(name.eq(storage_name)),
    )
    .execute(conn)?
        > 0)
}

/**
 * Returns the storage id based on the user id and storage name.
 */
pub fn get_id_storage(conn: &PgCon, id_user: i32, storage_name: &str) -> Result<i32, XamXamError> {
    Ok(storages
        .filter(user_id.eq(id_user))
        .filter(name.eq(storage_name))
        .select(id)
        .get_result::<i32>(conn)?)
}
