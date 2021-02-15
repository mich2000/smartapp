use serde::Deserialize;
use xam_xam_dal::enums::storage_kind::StorageKind;
use xam_xam_dal::models::storage::UpdateStorage;

#[derive(Deserialize)]
pub struct EditStorage {
    storage_name: String,
    new_storage_name: Option<String>,
    new_kind: StorageKind,
}

impl From<&EditStorage> for UpdateStorage {
    fn from(edit: &EditStorage) -> Self {
        UpdateStorage {
            name: edit.new_storage_name.clone(),
            storage_kind: Some(edit.new_kind.clone()),
        }
    }
}

impl EditStorage {
    pub fn get_storage_name(&self) -> &str {
        &self.storage_name
    }
}
