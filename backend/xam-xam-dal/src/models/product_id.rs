use diesel::sql_types::Integer;

#[derive(QueryableByName)]
pub struct ProductId {
    #[sql_type = "Integer"]
    product_id: i32,
}

impl ProductId {
    pub fn get_id(&self) -> i32 {
        self.product_id
    }
}
