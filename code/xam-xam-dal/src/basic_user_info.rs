use chrono::{NaiveDate,Utc};
use diesel::sql_types::{BigInt,Date,Nullable};

#[derive(Debug,QueryableByName,FromSqlRow)]
pub struct BasicUserInfo {
    #[sql_type = "BigInt"]
    pub amount_storage : i64,
    #[sql_type = "BigInt"]
    pub amount_product : i64,
    #[sql_type = "Nullable<Date>"]
    pub min_bederf : Option<NaiveDate>,
    #[sql_type = "Nullable<Date>"]
    pub max_bederf : Option<NaiveDate>
}

impl BasicUserInfo {
    pub fn new(amount_storage : i64,amount_product : i64,min_bederf : Option<NaiveDate>,max_bederf : Option<NaiveDate>) -> Self {
        Self {
            amount_storage,
            amount_product,
            min_bederf,
            max_bederf
        }
    }
}