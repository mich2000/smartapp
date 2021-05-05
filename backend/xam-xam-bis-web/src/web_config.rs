use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use env_logger::Builder;
use std::io::Write;

/**
 * Function that initializes an IdentityService
 */
pub fn identity() -> IdentityService<CookieIdentityPolicy> {
    IdentityService::new(
        CookieIdentityPolicy::new(&[0; 32])
            .name("Authorization")
            .http_only(true),
    )
}

/**
 * Function returning cors middleware
 */
pub fn cors() -> Cors {
    Cors::permissive().allowed_methods(vec!["POST", "PUT", "DELETE", "GET", "OPTIONS"])
}

/**
 * Function that is used to implement logging for the console in a pretty manner for easy reading.
 */
pub fn log_init() -> Result<(), Box<dyn std::error::Error>> {
    let env = env_logger::Env::default().filter_or("RUST_LOG", "info");
    Builder::from_env(env)
        .format(|buf, record| {
            let timestamp = buf.timestamp();
            writeln!(
                buf,
                "{}: {} => {:?}",
                record.level(),
                timestamp,
                record.args()
            )
        })
        .init();
    Ok(())
}
