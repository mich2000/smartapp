use actix_web::{App, HttpServer, middleware};

mod err;
mod controller; 

use xam_xam_id_bll::{PgPool,get_pg_pool};
use xam_xam_id_bll::{RedisPool,get_redis_pool};

#[actix_web::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let jwt_config = jwt_gang::from_env_config("Jwt.toml")?;
    let pg_pool : PgPool = get_pg_pool(&dotenv::var("DATABASE_URL").unwrap(), dotenv::var("DATABASE_NUM").unwrap().parse().unwrap());
    let mailer = mailgang::curl_mail::Mailer::default().unwrap();
    let redis_pool : RedisPool = get_redis_pool(&dotenv::var("REDIS_URL").unwrap(), dotenv::var("REDIS_URL_NUM").unwrap().parse().unwrap());
    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
        .data(pg_pool.clone())
        .data(mailer.clone())
        .app_data(jwt_config.clone())
    })
    .bind("0.0.0.0:8080")?
    .workers(1)
    .run()
    .await?;
    Ok(())
}