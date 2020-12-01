use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct IdProduct { 
    id : i32,
    storage_name : String
}

impl IdProduct {
    pub fn get_storage_name(&self) -> &str {
        &self.storage_name
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
}
