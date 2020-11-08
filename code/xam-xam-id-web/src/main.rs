use actix_web::{App, HttpServer, middleware,web,http::header::ContentEncoding};

mod err;
mod controllers;
mod db;
mod web_config;
mod extractors;

use xam_xam_id_bll::{PgPool,get_pg_pool,RedisPool,get_redis_pool};

#[macro_use] extern crate log;

#[actix_web::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    web_config::log_init()?;

    let pg_pool : PgPool = get_pg_pool(&dotenv::var("DATABASE_URL")?, dotenv::var("DATABASE_NUM")?.parse()?);
    let redis_pool : RedisPool = get_redis_pool(&dotenv::var("REDIS_URL")?, dotenv::var("REDIS_URL_NUM")?.parse()?);
    let jwt_config = jwt_gang::from_env_config("Jwt.toml")?;

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
                .service(controllers::request::request_new_email)
        )
        .service(
            web::scope("/auth")
                .service(controllers::auth::register)
                .service(controllers::auth::login)
                .service(controllers::auth::logout)
                .service(controllers::auth::renew_token)
                .service(controllers::auth::change_forgotten_pwd)
                .service(controllers::auth::validate)
        )
        .service(
            web::scope("/user")
                .service(controllers::user::get_basic_info)
                .service(controllers::user::change_email)
                .service(controllers::user::change_password)
                .service(controllers::user::delete_profile)
        )
        .default_service(web::route().to(web::HttpResponse::NotFound))
    })
    .bind_rustls("0.0.0.0:8080",web_config::tls_config())?
    .workers(1)
    .run()
    .await?;
    Ok(())
}