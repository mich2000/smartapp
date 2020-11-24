use serde::{Deserialize, Serialize};
use xam_xam_dal::enums::product_kind::ProductKind;
use chrono::NaiveDate;
use xam_xam_dal::models::product::Product;

/**
 * Struct representing a series of products. Struct unit inside vector represents => (product name, product amount, product peremption date, product kind)
 */
#[derive(Serialize, Deserialize)]
pub struct Products { 
    products : Vec<(String, i16, NaiveDate, ProductKind)>
}

impl From<Vec<Product>> for Products {
    fn from(list : Vec<Product>) -> Products {
        Products {
            products : list.iter().map(|product| (product.name.to_owned(), product.amount.to_owned(), product.peremption_date.to_owned(), product.product_kind.to_owned())).collect()
        }
    }
}