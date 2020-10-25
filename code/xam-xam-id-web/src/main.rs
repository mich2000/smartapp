use actix_web::{App, HttpServer, middleware,web};
use actix_identity::{CookieIdentityPolicy, IdentityService};

mod err;
mod controller;
mod db;

use xam_xam_id_bll::{PgPool,get_pg_pool};
use xam_xam_id_bll::{RedisPool,get_redis_pool};

#[macro_use] extern crate log;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

#[actix_web::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    log_init()?;

    let pg_pool : PgPool = get_pg_pool(&dotenv::var("DATABASE_URL")?, dotenv::var("DATABASE_NUM")?.parse()?);
    let redis_pool : RedisPool = get_redis_pool(&dotenv::var("REDIS_URL")?, dotenv::var("REDIS_URL_NUM")?.parse()?);

    HttpServer::new(move|| {
        App::new()
        .wrap(middleware::Logger::default())
        .wrap(IdentityService::new(CookieIdentityPolicy::new(&[0; 32]).name("Authorization").secure(true).http_only(true)))
        .data(pg_pool.clone())
        .data(redis_pool.clone())
        .data(mailgang::mailer_gang::Mailer::default())
        .app_data(jwt_gang::from_env_config("Jwt.toml").unwrap())
        .service(
            web::scope("/request")
                .service(controller::request_new_user)
        )
        .service(
            web::scope("/user")
                .service(controller::register)
        )
    })
    .bind("0.0.0.0:8080")?
    .workers(1)
    .run()
    .await?;
    Ok(())
}

fn log_init() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_config(
        Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(
            ConsoleAppender::builder()
                .encoder(Box::new(PatternEncoder::new("{l}: {d(%Y-%m-%d %H:%M:%S)} => {m}{n}")))
                .build()
        )))
        .build(
            Root::builder()
                .appender("stdout")
                .build(LevelFilter::Info)
        )?
    )?;
    Ok(())
}