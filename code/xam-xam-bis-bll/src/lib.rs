pub mod err;
pub mod service;
pub mod viewmodels;

#[macro_use]
extern crate log;

pub use xam_xam_dal::err::XamXamError;

/**
 * ======================= POSTGRES PART =======================
*/
pub type PgPool = xam_xam_dal::PostgresPool;
pub type PgCon = xam_xam_dal::PgCon;

pub fn get_pg_pool(url: &str, max_conn: u32) -> PgPool {
    xam_xam_dal::get_pool(url, max_conn)
}
