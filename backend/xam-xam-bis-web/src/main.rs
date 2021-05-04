mod controllers;
mod db;
mod err;
mod user_id;
mod web_config;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SECRET: String = get_value_from_key("JWT_SECRET")
        .ok_or(JwtCustomError::EnvironmentalVariableMissing)
        .unwrap();
}

use crate::err::XamXamWebError;
use actix_web::{http::header::ContentEncoding, middleware, web, App, HttpServer};
use jwt_gang::claim_error::JwtCustomError;
use xam_xam_bis_bll::{get_pg_pool, PgPool};
use xam_xam_common::util::get_value_from_key;

#[actix_web::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    web_config::log_init()?;
    let pg_pool: PgPool = get_pg_pool(
        &get_value_from_key("DATABASE_URL").ok_or(XamXamWebError::CouldNotGetPostGresConnection)?,
    );
    let jwt_config = jwt_gang::env_config(&SECRET)?;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::new(ContentEncoding::Gzip))
            .wrap(web_config::cors())
            .data(pg_pool.clone())
            .data(jwt_config.clone())
            .wrap(web_config::identity())
            .service(
                web::scope("/bis")
                .service(controllers::product::add_product)
                .service(controllers::product::remove_product)
                .service(controllers::product::update_product)
                .service(controllers::product::get_product_list)
                .service(controllers::product::move_product)
                .service(controllers::storage::add_storage)
                .service(controllers::storage::delete_storage)
                .service(controllers::storage::get_storages)
                .service(controllers::storage::edit_storage)
            )
            .default_service(web::route().to(web::HttpResponse::NotFound))
    })
    .bind_rustls("0.0.0.0:8081", web_config::tls_config())?
    .keep_alive(15)
    .workers(1)
    .run()
    .await?;
    Ok(())
}
