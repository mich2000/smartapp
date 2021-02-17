use crate::err::XamXamWebError;
use crate::{PgPool, RedisPool};
use actix_web::web::Data;
use xam_xam_id_bll::{PgCon, RCon};

pub trait GetCon<T> {
    fn conn(&self) -> Result<T, XamXamWebError>;
}

impl GetCon<PgCon> for Data<PgPool> {
    fn conn(&self) -> Result<PgCon, XamXamWebError> {
        self.get()
            .or(Err(XamXamWebError::CouldNotGetPostGresConnection))
    }
}

impl GetCon<RCon> for Data<RedisPool> {
    fn conn(&self) -> Result<RCon, XamXamWebError> {
        self.get()
            .or(Err(XamXamWebError::CouldNotGetRedisConnection))
    }
}
