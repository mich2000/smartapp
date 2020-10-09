use diesel_derive_enum::DbEnum;
/**
 * Enumeration containing all the kind of storage a storage can be.
 */
#[derive(Debug, PartialEq, DbEnum)]
pub enum StorageKind {
    Other,
    Closet,
    Fridge,
    Freezer
}