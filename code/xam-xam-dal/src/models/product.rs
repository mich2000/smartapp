use crate::schema::*;
use crate::enums::product_kind::ProductKind;
use crate::models::storage::Storage;
use chrono::{NaiveDate,Utc};
use crate::err::XamXamError;

/**
 * Struct that represents a product, this product has to be linked to a specific storage from a user.
 */
#[derive(Insertable, Queryable, Associations, Identifiable, Debug, PartialEq, Clone)]
#[belongs_to(Storage)]
#[table_name = "products"]
pub struct Product {
    pub id : i32,
    pub storage_id : i32,
    pub name : String,
    pub amount : i16,
    pub peremption_date: NaiveDate,
    pub product_kind : ProductKind
}

#[derive(AsChangeset)]
#[table_name = "products"]
pub struct UpdateProduct {
    pub name : Option<String>,
    pub amount : Option<i16>,
    pub peremption_date: Option<NaiveDate>,
    pub product_kind : Option<ProductKind>
}

/**
 * Struct that is used to insert a product into the database
 */
#[derive(Insertable,Debug)]
#[table_name = "products"]
pub struct InsertableProduct {
    pub storage_id : i32,
    pub name : String,
    pub amount : i16,
    pub peremption_date: NaiveDate,
    pub product_kind : ProductKind
}

impl InsertableProduct {
    /**
     * Creates a new InsertableProduct object, where the peremption date and product kind is optional. If the peremption date is empty, then it will give the date of today to the struct. If the ProductKind is empty, the default enum value will be used then this is the Other value then.
     */
    pub fn new(storage_id : i32, product_name : &str, amount : i16, peremption_date : Option<NaiveDate>, product_kind : Option<ProductKind>) -> Result<InsertableProduct, XamXamError> {
        if product_name.is_empty() {
            return Err(XamXamError::ProductNameIsEmpty)
        }
        Ok(
            Self {
                storage_id : storage_id,
                name : product_name.to_string(),
                amount : amount,
                peremption_date : peremption_date.unwrap_or_else(|| Utc::now().naive_local().date()),
                product_kind : product_kind.unwrap_or(ProductKind::Other)
            }
        )
    }
}