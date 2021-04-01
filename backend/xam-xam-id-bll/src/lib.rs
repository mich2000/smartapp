pub mod auth_service;
pub mod err;
pub mod viewmodels;

pub use xam_xam_dal::err::XamXamError;

#[macro_use]
extern crate log;

use r2d2_redis::{r2d2::Pool, RedisConnectionManager};

/**
 * ======================= POSTGRES PART =======================
*/
pub type PgPool = xam_xam_dal::PostgresPool;
pub type PgCon = xam_xam_dal::PgCon;

pub fn get_pg_pool(url: &str) -> PgPool {
    xam_xam_dal::get_pool(url)
}

/**
 * ======================= REDIS PART =======================
*/
pub type RedisPool = Pool<RedisConnectionManager>;
pub type RCon = r2d2::PooledConnection<RedisConnectionManager>;

pub fn get_redis_pool(url: &str) -> RedisPool {
    let manager = RedisConnectionManager::new(url).unwrap();
    Pool::builder().max_size(1).build(manager).unwrap()
}
