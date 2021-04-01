#[derive(serde::Serialize)]
pub struct StorageNames(Vec<String>);

impl From<Vec<String>> for StorageNames {
    fn from(names: Vec<String>) -> Self {
        Self(names)
    }
}
