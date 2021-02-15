use crate::err::XamXamWebError;
use actix_web::web::Data;
use xam_xam_bis_bll::{PgCon, PgPool};

pub trait GetCon<T> {
    fn conn(&self) -> Result<T, XamXamWebError>;
}

impl GetCon<PgCon> for Data<PgPool> {
    fn conn(&self) -> Result<PgCon, XamXamWebError> {
        self.get()
            .or(Err(XamXamWebError::CouldNotGetPostGresConnection))
    }
}
