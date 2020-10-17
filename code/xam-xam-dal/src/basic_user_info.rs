use chrono::NaiveDate;

pub struct BasicUserInfo {
    amount_storage : i16,
    amount_product : i16,
    min_bederf : Option<NaiveDate>,
    max_bederf : Option<NaiveDate>
}

impl BasicUserInfo {
    pub fn new(amount_storage : i16,amount_product : i16,min_bederf : Option<NaiveDate>,max_bederf : Option<NaiveDate>) -> Self {
        Self {
            amount_storage,amount_product,min_bederf,max_bederf
        }
    }
}