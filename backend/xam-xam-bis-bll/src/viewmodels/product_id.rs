use serde::Serialize;

#[derive(Serialize)]
pub struct ProductId {
    product_id : i32
}

impl ProductId {
    pub fn new(id : i32) -> Self {
        ProductId {
            product_id : id
        }
    }
    pub fn get_id(&self) -> i32 {
        self.product_id
    }
}