use serde::{Deserialize, Serialize};
use xam_xam_dal::enums::product_kind::ProductKind;
use chrono::NaiveDate;

#[derive(Serialize, Deserialize)]
pub struct UpdateProduct {
    storage_name: String,
    name: Option<String>,
    amount: Option<i16>,
    peremption_date: Option<NaiveDate>,
    kind: ProductKind
}

impl UpdateProduct {
    pub fn get_storage_name(&self) -> &str { &self.storage_name }

    pub fn get_name(&self) -> Option<&str> { self.name }
    
    pub fn get_amount(&self) -> Option<i16> { self.amount }

    pub fn get_peremption_date(&self) -> Option<NaiveDate> { self.peremption_date }

    pub fn get_kind(&self) -> ProductKind { self.kind.to_owned() }
}
