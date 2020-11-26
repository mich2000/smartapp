use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
pub struct StorageName {
    name : String
}

impl StorageName {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}