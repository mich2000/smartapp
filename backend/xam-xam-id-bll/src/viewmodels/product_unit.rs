use xam_xam_dal::product_description::ProductDescription;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct ProductUnits {
    units: Vec<ProductDescription>
}

impl From<Vec<ProductDescription>> for ProductUnits {
    fn from(units : Vec<ProductDescription>) -> Self {
        Self {
            units
        }
    }
}