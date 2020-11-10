use crate::err::XamXamWebError;
use actix_web::web::Data;
use xam_xam_bis_bll::{PgCon,PgPool};

pub fn get_pg_conn(pg: Data<PgPool>) -> Result<PgCon, XamXamWebError> {
    match pg.get() {
        Ok(conn) => Ok(conn),
        Err(_) => Err(XamXamWebError::CouldNotGetPostGresConnection),
    }
}