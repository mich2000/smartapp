use serde::{Deserialize, Serialize};
use xam_xam_dal::enums::product_kind::ProductKind;
use chrono::NaiveDate;
use crate::date_ser;

#[derive(Serialize, Deserialize,Debug)]
pub struct InsertProduct {
    storage_name: String,
    name: String,
    amount: i32,
    #[serde(with = "date_ser")]
    peremption_date: NaiveDate,
    kind: ProductKind
}

use std::str::FromStr;

impl Default for InsertProduct {
    fn default() -> Self {
        Self {
            storage_name : "storage".to_owned(),
            name : "name".to_owned(),
            amount : 1,
            peremption_date : NaiveDate::from_str("2020-11-11").unwrap(),
            kind : ProductKind::Other
        }
    }
}

impl InsertProduct {
    pub fn get_storage_name(&self) -> &str { &self.storage_name }

    pub fn get_name(&self) -> &str { &self.name }
    
    pub fn get_amount(&self) -> i32 { self.amount }

    pub fn get_peremption_date(&self) -> &NaiveDate { &self.peremption_date }

    pub fn get_kind(&self) -> ProductKind { self.kind.clone() }
}