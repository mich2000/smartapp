pub mod err;
pub mod viewmodels;
pub mod auth_service;

#[macro_use] extern crate log;

use r2d2_redis::{r2d2::Pool, RedisConnectionManager};

/**
 * ======================= POSTGRES PART =======================
*/
pub type PgPool = xam_xam_dal::PostgresPool;
pub type PgCon = xam_xam_dal::PgCon;

pub fn get_pg_pool(url : &str, max_conn : u32) -> PgPool {
    xam_xam_dal::get_pool(url,max_conn)
}

/**
 * ======================= REDIS PART =======================
*/
pub type RedisPool = Pool<RedisConnectionManager>;
pub type R2D2Con = r2d2::PooledConnection<RedisConnectionManager>;

pub fn get_redis_pool(url : &str, max_conn : u32) -> RedisPool {
    let manager = RedisConnectionManager::new(url).unwrap();
    Pool::builder().max_size(max_conn).build(manager).unwrap()
}