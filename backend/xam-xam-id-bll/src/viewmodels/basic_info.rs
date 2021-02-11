use chrono::NaiveDate;
use serde::Serialize;
use xam_xam_dal::models::basic_user_info::BasicUserInfo;

#[derive(Serialize)]
pub struct UserInfo {
    amount_storage : i16,
    amount_product : i16,
    min_bederf : Option<NaiveDate>,
    max_bederf : Option<NaiveDate>
}

impl UserInfo {
    pub fn new(model : &BasicUserInfo) -> Self {
        Self {
            amount_storage : model.amount_storage,
            amount_product : model.amount_product,
            min_bederf : model.min_bederf,
            max_bederf : model.max_bederf
        }
    }
}