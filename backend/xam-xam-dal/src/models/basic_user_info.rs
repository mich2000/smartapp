use chrono::NaiveDate;
use diesel::sql_types::{Int4, Date, Nullable};

#[derive(QueryableByName)]
pub struct BasicUserInfo {
    #[sql_type = "Int4"]
    pub amount_storage: i32,
    #[sql_type = "Int4"]
    pub amount_product: i32,
    #[sql_type = "Nullable<Date>"]
    pub min_bederf: Option<NaiveDate>,
    #[sql_type = "Nullable<Date>"]
    pub max_bederf: Option<NaiveDate>,
}
