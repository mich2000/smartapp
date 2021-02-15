use chrono::NaiveDate;
use serde::Deserialize;
use xam_xam_dal::enums::product_kind::ProductKind;

#[derive(Deserialize)]
pub struct InsertProduct {
    storage_name: String,
    name: String,
    amount: i16,
    peremption_date: NaiveDate,
    kind: ProductKind,
}

impl InsertProduct {
    pub fn get_storage_name(&self) -> &str {
        &self.storage_name
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_amount(&self) -> i16 {
        self.amount
    }

    pub fn get_peremption_date(&self) -> &NaiveDate {
        &self.peremption_date
    }

    pub fn get_kind(&self) -> ProductKind {
        self.kind.clone()
    }
}
