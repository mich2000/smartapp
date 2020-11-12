mod db;
mod err;
mod user_id;
mod web_config;
mod controllers;

#[macro_use] extern crate log;

use actix_web::{http::header::ContentEncoding, middleware, web, App, HttpServer};

use xam_xam_bis_bll::{PgPool,PgCon,get_pg_pool};

#[actix_web::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    web_config::log_init()?;

    let pg_pool: PgPool = get_pg_pool(
        &dotenv::var("DATABASE_URL")?,
        dotenv::var("DATABASE_NUM")?.parse()?,
    );
    let jwt_config = jwt_gang::from_env_config("Jwt.toml")?;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::new(ContentEncoding::Gzip))
            .wrap(web_config::cors())
            .data(pg_pool.clone())
            .data(jwt_config.clone())
            .wrap(web_config::identity())
            .service(controllers::storage::add_storage)
            .service(controllers::storage::delete_storage)
            .service(controllers::storage::get_storages)
            .default_service(web::route().to(web::HttpResponse::NotFound))
    })
    .bind_rustls("0.0.0.0:8081", web_config::tls_config())?
    .workers(1)
    .run()
    .await?;
    Ok(())
}
