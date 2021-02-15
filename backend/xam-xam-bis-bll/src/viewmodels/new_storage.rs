use serde::Deserialize;
use xam_xam_dal::enums::storage_kind::StorageKind;

#[derive(Deserialize)]
pub struct NewStorage {
    name: String,
    kind: StorageKind,
}

impl NewStorage {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_kind(&self) -> StorageKind {
        self.kind.clone()
    }
}
