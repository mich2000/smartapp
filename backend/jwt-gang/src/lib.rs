pub mod claim;
pub mod claim_config;
pub mod claim_error;
mod jwt_numeric_date;

#[macro_use]
extern crate log;

use claim_config::ClaimConfiguration;
use claim_error::JwtCustomError;

pub fn get_value_from_key(key: &str) -> Option<String> {
    match dotenv::var(key) {
        Ok(value) => Some(value),
        Err(_) => match std::env::var(key) {
            Ok(env_value) => Some(env_value),
            Err(_) => None,
        },
    }
}

pub fn env_config(secret: &'static str) -> Result<ClaimConfiguration<'_>, JwtCustomError> {
    Ok(ClaimConfiguration::new(
        &get_value_from_key("JWT_ISSUER").ok_or(JwtCustomError::EnvironmentalVariableMissing)?,
        //&get_value_from_key("JWT_SECRET").ok_or(JwtCustomError::EnvironmentalVariableMissing)?,
        secret,
        get_value_from_key("JWT_EXPIRATION")
            .ok_or(JwtCustomError::EnvironmentalVariableMissing)?
            .parse::<u64>()
            .map_err(|_| {
                JwtCustomError::CustomError("Cannot parse the jwt expiration env line".to_owned())
            })?,
    ))
}

#[test]
fn test_good_claim() {
    use claim_config::ClaimConfiguration;
    let config = ClaimConfiguration::new("i", "s", 15);
    let claim = config.create_claim("u").unwrap();
    let token = config.token_from_claim(&claim).unwrap();
    assert!(config.decode_token(&token).is_ok());
    assert!(config.decode_token(&token).unwrap().claims.get_subject() == "u");
}

#[test]
fn test_expiration() {
    use claim_config::ClaimConfiguration;
    let config = ClaimConfiguration::new("i", "s", 15);
    let claim = config.create_claim("u");
    assert!(!claim.is_err())
}

#[test]
fn test_expiration_after_sleeping() {
    use claim_config::ClaimConfiguration;
    let config = ClaimConfiguration::new("i", "s", 1);
    let claim = config.create_claim("u");
    assert!(claim.is_ok());
    let token = config.token_from_claim(&claim.unwrap()).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(5));
    assert!(config.decode_token(&token).is_err());
}