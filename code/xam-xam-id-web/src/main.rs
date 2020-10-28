use actix_web::{App, HttpServer, middleware,web,http::header::ContentEncoding};

mod err;
mod controller;
mod db;
mod web_config;
mod extractor;

use xam_xam_id_bll::{PgPool,get_pg_pool};
use xam_xam_id_bll::{RedisPool,get_redis_pool};

#[macro_use] extern crate log;

#[actix_web::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    web_config::log_init()?;

    let pg_pool : PgPool = get_pg_pool(&dotenv::var("DATABASE_URL")?, dotenv::var("DATABASE_NUM")?.parse()?);
    let redis_pool : RedisPool = get_redis_pool(&dotenv::var("REDIS_URL")?, dotenv::var("REDIS_URL_NUM")?.parse()?);
    let jwt_config = jwt_gang::from_env_config("Jwt.toml")?;

    HttpServer::new(move|| {
        App::new()
        .data(web::JsonConfig::default().limit(4096))
        .data(pg_pool.clone())
        .data(redis_pool.clone())
        .data(mailgang::mailer_gang::Mailer::default())
        .data(jwt_config.clone())
        .wrap(middleware::Logger::default())
        .wrap(middleware::Compress::new(ContentEncoding::Gzip))
        .wrap(web_config::identity())
        .wrap(web_config::cors())
        .service(
            web::scope("/request")
                .service(controller::request_new_user)
        )
        .service(
            web::scope("/auth")
                .service(controller::register)
                .service(controller::login)
                .service(controller::logout)
        )
        .service(
            web::scope("/user")
                .service(controller::get_basic_info)
        )
        .default_service(web::route().to(web::HttpResponse::NotFound))
    })
    .bind_rustls("0.0.0.0:8080",web_config::tls_config())?
    .workers(1)
    .keep_alive(None)
    .run()
    .await?;
    Ok(())
}