use chrono::NaiveDate;
use diesel::sql_types::{SmallInt, Date, Nullable};

#[derive(QueryableByName)]
pub struct BasicUserInfo {
    #[sql_type = "SmallInt"]
    pub amount_storage: i16,
    #[sql_type = "SmallInt"]
    pub amount_product: i16,
    #[sql_type = "Nullable<Date>"]
    pub min_bederf: Option<NaiveDate>,
    #[sql_type = "Nullable<Date>"]
    pub max_bederf: Option<NaiveDate>,
}
