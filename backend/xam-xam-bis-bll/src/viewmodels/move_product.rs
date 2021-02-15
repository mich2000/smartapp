use serde::Deserialize;

#[derive(Deserialize)]
pub struct MoveProduct {
    product_id : i32,
    new_storage_name : String
}

impl MoveProduct {
    pub fn get_product_id(&self) -> i32 {
        self.product_id
    }
    
    pub fn get_new_storage_name(&self) -> &str {
        &self.new_storage_name
    }
}