use chrono::NaiveDate;
use diesel::sql_types::{Bigint, Date, Nullable};

#[derive(QueryableByName)]
pub struct BasicUserInfo {
    #[sql_type = "Bigint"]
    pub amount_storage: i64,
    #[sql_type = "Bigint"]
    pub amount_product: i64,
    #[sql_type = "Nullable<Date>"]
    pub min_bederf: Option<NaiveDate>,
    #[sql_type = "Nullable<Date>"]
    pub max_bederf: Option<NaiveDate>,
}
