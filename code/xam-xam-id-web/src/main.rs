use actix_web::{App, HttpServer};

mod db;
mod err;

#[actix_web::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let jwt_config = jwt_gang::from_env_config("Jwt.toml")?;
    HttpServer::new(move || {
        App::new()
        .app_data(jwt_config.clone())
    })
    .bind("127.0.0.1:8080")?
    .workers(1)
    .run()
    .await?;
    Ok(())
}