use diesel_derive_enum::DbEnum;

/**
 * Enumeration containing all the kind of storage a storage can be.
 */
#[derive(Debug, DbEnum,PartialEq,Clone)]
#[DieselType = "Storage_Kind"]
pub enum StorageKind {
    Other,
    Closet,
    Fridge,
    Freezer
}