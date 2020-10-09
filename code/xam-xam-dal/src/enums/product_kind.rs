/**
 * Enumeration containing all the kind of food a product can be.
 */
#[derive(Debug, PartialEq, DbEnum)]
#[DieselType = "ProductKind"]
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