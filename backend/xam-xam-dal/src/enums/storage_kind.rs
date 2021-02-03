use diesel_derive_enum::DbEnum;
use serde::{Serialize,Deserialize};

/**
 * Enumeration containing all the kind of storage a storage can be.
 */
#[derive(Debug, DbEnum, Clone, Serialize,Deserialize)]
#[DieselType = "Storage_Kind"]
pub enum StorageKind {
    Other,
    Closet,
    Fridge,
    Freezer
}
