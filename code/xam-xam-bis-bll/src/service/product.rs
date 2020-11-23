use xam_xam_dal::models::product::{InsertableProduct};
use crate::err::XamXamServiceError;
use crate::PgCon;

pub fn add_product(_: &PgCon,_ : &InsertableProduct) -> Result<(),XamXamServiceError> {

    Ok(())
}