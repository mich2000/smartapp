use crate::err::XamXamWebError;
use crate::{PgPool, RedisPool};
use actix_web::web::Data;
use xam_xam_id_bll::{RCon,PgCon};

pub fn get_pg_conn(pg : Data<PgPool>) -> Result<PgCon,XamXamWebError> {
    match pg.get() {
        Ok(conn) => Ok(conn),
        Err(_) => Err(XamXamWebError::CouldNotGetPostGresConnection)
    }
}

pub fn get_redis_conn(redis_db : Data<RedisPool>) -> Result<RCon,XamXamWebError> {
    match redis_db.get() {
        Ok(conn) => Ok(conn),
        Err(_) => Err(XamXamWebError::CouldNotGetRedisConnection)
    }
}