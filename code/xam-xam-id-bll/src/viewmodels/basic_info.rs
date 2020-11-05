use chrono::NaiveDate;
use serde::{Serialize,Deserialize};
use xam_xam_dal::basic_user_info::BasicUserInfo;

#[derive(Debug,Serialize,Deserialize)]
pub struct UserInfo {
    email : String,
    amount_storage : i64,
    amount_product : i64,
    min_bederf : Option<NaiveDate>,
    max_bederf : Option<NaiveDate>
}

impl UserInfo {
    pub fn new(model : &BasicUserInfo) -> Self {
        Self {
            email : model.email.to_owned(),
            amount_storage : model.amount_storage,
            amount_product : model.amount_product,
            min_bederf : model.min_bederf,
            max_bederf : model.max_bederf
        }
    }
}