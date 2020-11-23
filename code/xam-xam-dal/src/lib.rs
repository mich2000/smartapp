#[macro_use]
extern crate log;

#[macro_use]
pub extern crate diesel;
pub mod basic_user_info;
pub mod err;
pub mod models;
pub mod repo;
pub mod schema;
pub mod enums;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;
pub type PgCon = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

/**
 * Code I got from this website: https://github.com/lucperkins/rust-graphql-juniper-actix-diesel-postgres/blob/master/src/db.rs
 *
 * returns a pool that I can use, for my postgresql connections.
 */
pub fn get_pool(url: &str) -> PostgresPool {
    let mgr = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .max_size(1)
        .build(mgr)
        .expect("could not build connection pool")
}
