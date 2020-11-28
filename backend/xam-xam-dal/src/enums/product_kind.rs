use diesel_derive_enum::DbEnum;
use std::fmt;
use serde::{Deserialize, Serialize};

/**
 * Enumeration containing all the kind of food a product can be.
 */
#[derive(DbEnum, Debug, PartialEq, Clone, Deserialize, Serialize)]
#[DieselType = "Product_Kind"]
pub enum ProductKind {
    Other,
    Vegetables,
    Fruit,
    Grain,
    Meat,
    Fish,
    Dairy,
    Unhealthy,
    Bean,
}

impl std::fmt::Display for ProductKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",
            match self {
                ProductKind::Other => "other",
                ProductKind::Vegetables => "vegetables",
                ProductKind::Fruit => "fruit",
                ProductKind::Grain => "grain",
                ProductKind::Meat => "meat",
                ProductKind::Fish => "fish",
                ProductKind::Dairy => "diary",
                ProductKind::Unhealthy => "unhealthy",
                ProductKind::Bean => "bean"
            }
        )
    }
}