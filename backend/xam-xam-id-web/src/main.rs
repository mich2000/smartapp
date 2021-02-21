use actix_web::{http::header::ContentEncoding, middleware, web, App, HttpServer};

mod controllers;
mod db;
mod err;
mod extractors;
mod web_config;

use xam_xam_id_bll::{get_pg_pool, get_redis_pool, PgPool, RedisPool};

use xam_xam_common::util::get_value_from_key;

use crate::err::XamXamWebError;

#[macro_use]
extern crate log;

#[actix_web::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    web_config::log_init()?;

    let pg_pool: PgPool = get_pg_pool(
        get_value_from_key("DATABASE_URL")
            .ok_or(XamXamWebError::CouldNotGetPostGresConnection)?
            .as_ref(),
    );
    let redis_pool: RedisPool = get_redis_pool(
        &get_value_from_key("REDIS_URL")
            .ok_or(XamXamWebError::CouldNotGetRedisConnection)?
            .as_ref(),
    );
    let jwt_config = jwt_gang::env_config()?;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::new(ContentEncoding::Gzip))
            .wrap(web_config::cors())
            .data(pg_pool.clone())
            .data(redis_pool.clone())
            .data(mailgang::mailer_gang::Mailer::default())
            .data(jwt_config.clone())
            .wrap(web_config::identity())
            .service(
                web::scope("/request")
                    .service(controllers::request::request_new_user)
                    .service(controllers::request::request_pwd_change)
                    .service(controllers::request::request_new_email),
            )
            .service(
                web::scope("/auth")
                    .service(controllers::auth::register)
                    .service(controllers::auth::login)
                    .service(controllers::auth::logout)
                    .service(controllers::auth::renew_token)
                    .service(controllers::auth::change_forgotten_pwd)
                    .service(controllers::auth::validate)
                    .service(controllers::auth::get_email),
            )
            .service(
                web::scope("/user")
                    .service(controllers::user::get_basic_info)
                    .service(controllers::user::get_five_first_products)
                    .service(controllers::user::change_email)
                    .service(controllers::user::change_password)
                    .service(controllers::user::delete_profile),
            )
            .default_service(web::route().to(web::HttpResponse::NotFound))
    })
    .bind_rustls("0.0.0.0:8080", web_config::tls_config())?
    .keep_alive(15)
    .workers(1)
    .run()
    .await?;
    Ok(())
}
