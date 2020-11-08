use rustls::{NoClientAuth, ServerConfig, internal::pemfile::{certs, pkcs8_private_keys}};
use std::fs::File;
use std::io::BufReader;
use actix_cors::Cors;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use actix_identity::{CookieIdentityPolicy, IdentityService};

/**
 * Function that initializes an IdentityService
 */
pub fn identity() -> IdentityService<CookieIdentityPolicy> {
    IdentityService::new(
        CookieIdentityPolicy::new(&[0; 32])
            .name("Authorization")
            .max_age(60)
            .secure(true)
            .http_only(true)
    )
}

/**
 * Function returning cors middleware
 */
pub fn cors() -> Cors {
    Cors::permissive()
    .max_age(60)
    .allowed_methods(vec!["POST","PUT","DELETE","GET"])
}

/**
 * Tls configuration that is used to provide tls and https, this is important for security.
 */
pub fn tls_config() -> ServerConfig {
    let cert_file = &mut BufReader::new(File::open(dotenv::var("CERT_PATH").unwrap()).unwrap());
    let key_file = &mut BufReader::new(File::open(dotenv::var("KEY_PATH").unwrap()).unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    let mut config = ServerConfig::new(NoClientAuth::new());
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();
    config
}

/**
 * Function that is used to implement logging for the console in a pretty manner for easy reading.
 */
pub fn log_init() -> Result<(), Box<dyn std::error::Error>> {
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