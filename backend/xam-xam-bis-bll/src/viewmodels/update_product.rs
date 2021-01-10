use serde::Deserialize;
use xam_xam_dal::enums::product_kind::ProductKind;
use chrono::NaiveDate;

#[derive(Deserialize)]
pub struct UpdateProduct {
    id : i32,
    storage_name: String,
    name: Option<String>,
    amount: Option<i16>,
    peremption_date: Option<NaiveDate>,
    kind: ProductKind
}

impl From<&UpdateProduct> for xam_xam_dal::models::product::UpdateProduct {
    fn from(model : &UpdateProduct) -> Self {
        Self {
            id : model.get_id(),
            name: model.get_name(),
            amount: model.get_amount(),
            peremption_date: model.get_peremption_date(),
            product_kind: Some(model.get_kind()),
        }
    }
}

impl UpdateProduct {
    pub fn get_id(&self) -> i32 { self.id }

    pub fn get_storage_name(&self) -> &str { &self.storage_name }

    pub fn get_name(&self) -> Option<String> { self.name.clone() }
    
    pub fn get_amount(&self) -> Option<i16> { self.amount }

    pub fn get_peremption_date(&self) -> Option<NaiveDate> { self.peremption_date }

    pub fn get_kind(&self) -> ProductKind { self.kind.to_owned() }
}
