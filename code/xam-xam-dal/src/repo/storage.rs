use diesel::PgConnection;
use crate::err::XamXamError;
use crate::models::storage::{InsertableStorage,Storage,UpdateStorage};
use crate::schema::storages::dsl::*;
use crate::schema::storages::*;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use crate::diesel::query_dsl::filter_dsl::FindDsl;


/**
 * Inserts a storage in a database
 */
pub fn insert_storage(conn : &PgConnection, storage : &InsertableStorage) -> Result<(),XamXamError> {
    match diesel::insert_into(storages).values(storage).get_result::<Storage>(conn) {
        Ok(inserted_storage) => {
            info!("The storage {} has been added to the user with id {}",&inserted_storage.name, &inserted_storage.user_id);
            Ok(())
        },
        Err(e) => Err(e.into())
    }
}