use xam_xam_dal::models::product_description::ProductDescription;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProductUnits {
    units: Vec<ProductDescription>
}

impl From<Vec<ProductDescription>> for ProductUnits {
    fn from(units : Vec<ProductDescription>) -> Self {
        Self { units }
    }
}