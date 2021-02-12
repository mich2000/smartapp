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

#[test]
fn test_product_size_units() {
    println!("{}", std::mem::size_of::<Vec<ProductDescription>>());
    println!("{}", std::mem::size_of::<ProductDescription>());
    println!("{}", std::mem::size_of::<[Option<ProductDescription>;5]>());
}   