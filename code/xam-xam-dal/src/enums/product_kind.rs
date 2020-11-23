use diesel_derive_enum::DbEnum;
use std::fmt;

/**
 * Enumeration containing all the kind of food a product can be.
 */
#[derive(DbEnum, Debug, PartialEq, Clone)]
#[DieselType = "Product_Kind"]
pub enum ProductKind {
    Other,
    Vegetables,
    Fruit,
    Grain,
    Meat,
    Fish,
    Dairy,
    FatAndSugar,
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
                ProductKind::FatAndSugar => "fat_and_sugar",
                ProductKind::Bean => "bean"
            }
        )
    }
}