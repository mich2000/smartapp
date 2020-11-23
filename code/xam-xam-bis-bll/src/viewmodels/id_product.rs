use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IdProduct(i32);

impl IdProduct {
    pub fn get_id(&self) -> i32 {
        self.0
    }
}
