use serde::{Serialize,Deserialize};
use xam_xam_dal::models::storage::Storage;
use xam_xam_dal::enums::storage_kind::StorageKind;

#[derive(Serialize, Deserialize)]
pub struct Storages {
    storages : Vec<(String, StorageKind)>
}

impl From<Vec<Storage>> for Storages {
    fn from(vec_storages : Vec<Storage>) -> Storages {
        Storages {
            storages : vec_storages.iter().map(|storage| (storage.name.to_owned(),storage.storage_kind.clone())).collect()
        }
    }
}