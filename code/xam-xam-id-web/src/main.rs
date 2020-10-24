use actix_web::{App, HttpServer, middleware};

mod err;
mod controller; 

use xam_xam_id_bll::{PgPool,get_pg_pool};
use xam_xam_id_bll::{RedisPool,get_redis_pool};

use std::sync::Arc;

#[macro_use] extern crate log;

pub type ArcMailer = Arc<mailgang::mailer_gang::Mailer>;

#[actix_web::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    
    let jwt_config = jwt_gang::from_env_config("Jwt.toml")?;
    let pg_pool : PgPool = get_pg_pool(&dotenv::var("DATABASE_URL").unwrap(), dotenv::var("DATABASE_NUM").unwrap().parse().unwrap());
    let redis_pool : RedisPool = get_redis_pool(&dotenv::var("REDIS_URL").unwrap(), dotenv::var("REDIS_URL_NUM").unwrap().parse().unwrap());

    HttpServer::new(move|| {
        App::new()
        .wrap(middleware::Logger::default())
        .wrap(middleware::Logger::new("%a %{User-Agent}i"))
        .data(pg_pool.clone())
        .data(redis_pool.clone())
        .data(Arc::new(mailgang::mailer_gang::Mailer::default()))
        .app_data(jwt_config.clone())
        .service(controller::request_new_user)
        .service(controller::lol)
    })
    .bind("127.0.0.1:8080")?
    .workers(5)
    .run()
    .await?;
    Ok(())
}