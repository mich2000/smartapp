use xam_xam_dal::models::product_description::ProductDescription;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProductUnits {
    units: [Option<ProductDescription>;5]
}

impl From<Vec<ProductDescription>> for ProductUnits {
    fn from(units : Vec<ProductDescription>) -> Self {
        let mut product_arr : [Option<ProductDescription>;5] = [None,None,None,None,None];
        for index in 0..units.len() {
            product_arr[index] = Some(units[index].clone());
        }
        Self { units : product_arr }
    }
}

#[test]
fn test_product_size_units() {
    println!("{}", std::mem::size_of::<Vec<ProductDescription>>());
    println!("{}", std::mem::size_of::<ProductDescription>());
    println!("{}", std::mem::size_of::<[Option<ProductDescription>;5]>());
}   