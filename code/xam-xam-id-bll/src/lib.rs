pub mod err;
pub mod viewmodels;
pub mod auth_service;

pub type PgPool = xam_xam_dal::PostgresPool;

pub fn get_pg_pool(url : &str, max_conn : u32) -> PgPool {
    xam_xam_dal::get_pool(url,max_conn)
}