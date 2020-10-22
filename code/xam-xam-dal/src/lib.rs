mod enums;

#[macro_use] extern crate log;

#[macro_use] pub extern crate diesel;
pub mod schema;
pub mod models;
pub mod err;
pub mod repo;
pub mod basic_user_info;


use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

/**
 * Code I got from this website: https://github.com/lucperkins/rust-graphql-juniper-actix-diesel-postgres/blob/master/src/db.rs
 * 
 * returns a pool that I can use, for my postgresql connections.
 */
pub fn get_pool(url : &str, max_connections : u32) -> PostgresPool {
    let mgr = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .max_size(max_connections)
        .build(mgr)
        .expect("could not build connection pool")
}