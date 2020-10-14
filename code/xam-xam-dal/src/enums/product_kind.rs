use diesel_derive_enum::DbEnum;

/**
 * Enumeration containing all the kind of food a product can be.
 */
#[derive(DbEnum,Debug,PartialEq,Clone)]
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
    Bean
}