use chrono::NaiveDate;
use crate::enums::product_kind::ProductKind;
use diesel::sql_types::{SmallInt, Date, Text};
use serde::{Serialize,Deserialize};

#[derive(Debug, QueryableByName, FromSqlRow, Serialize, Deserialize)]
pub struct ProductDescription {
    #[sql_type = "Text"]
    name : String,
    #[sql_type = "SmallInt"]
    amount : i16,
    #[sql_type = "Date"]
    date : NaiveDate,
    #[sql_type = "Text"]
    kind : ProductKind,
    #[sql_type = "Text"]
    storage_name : String
}

impl diesel::deserialize::FromSql<diesel::sql_types::Text, diesel::pg::Pg> for ProductKind {
    fn from_sql(bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
        match String::from_utf8_lossy(bytes.unwrap_or_default()).as_ref() {
            "other" => Ok(ProductKind::Other),
            "vegetables" => Ok(ProductKind::Vegetables),
            "fruit" => Ok(ProductKind::Fruit),
            "grain" => Ok(ProductKind::Grain),
            "meat" => Ok(ProductKind::Meat),
            "fish" => Ok(ProductKind::Fish),
            "dairy" => Ok(ProductKind::Dairy),
            "unhealthy" => Ok(ProductKind::Unhealthy),
            "bean" => Ok(ProductKind::Bean),
            _ => Err("Unrecognized variant".into()),
        }
    }
}