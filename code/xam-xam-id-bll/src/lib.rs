pub mod err;
pub mod viewmodels;
pub mod auth_service;

use r2d2_redis::{r2d2::Pool, RedisConnectionManager};

pub type PgPool = xam_xam_dal::PostgresPool;

pub fn get_pg_pool(url : &str, max_conn : u32) -> PgPool {
    xam_xam_dal::get_pool(url,max_conn)
}

pub type RedisPool = Pool<RedisConnectionManager>;

pub fn get_redis_pool(url : &str, max_conn : u32) -> RedisPool {
    let manager = RedisConnectionManager::new(url).unwrap();
    let pool = Pool::builder()
        .max_size(max_conn)
        .build(manager)
        .unwrap();
    pool
}