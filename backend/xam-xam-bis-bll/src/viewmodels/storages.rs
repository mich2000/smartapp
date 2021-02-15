use serde::Serialize;
use xam_xam_dal::enums::storage_kind::StorageKind;
use xam_xam_dal::models::storage::Storage;

#[derive(Serialize)]
pub struct Storages {
    storages: Vec<(String, StorageKind)>,
}

impl From<Vec<Storage>> for Storages {
    fn from(storages: Vec<Storage>) -> Storages {
        Storages {
            storages: storages
                .iter()
                .map(|storage| (storage.name.to_owned(), storage.storage_kind.clone()))
                .collect(),
        }
    }
}
