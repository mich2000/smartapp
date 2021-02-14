use xam_xam_dal::models::product_description::ProductDescription;
use serde::Serialize;
use smallvec::SmallVec;

#[derive(Serialize)]
pub struct ProductUnits {
    units: SmallVec::<[ProductDescription;5]>
}

impl From<Vec<ProductDescription>> for ProductUnits {
    fn from(units : Vec<ProductDescription>) -> Self {
        Self { units : SmallVec::from(units) }
    }
}

#[test]
fn test_product_size_units() {
    println!("{}", std::mem::size_of::<Vec<ProductDescription>>());
    println!("{}", std::mem::size_of::<ProductDescription>());
    println!("{}", std::mem::size_of::<[Option<ProductDescription>;5]>());
}   