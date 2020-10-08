mod claim_config;
mod claim;
mod jwt_numeric_date;
mod claim_error;

use claim_config::ClaimConfiguration;
use claim_error::JwtCustomError;
use std::fs;

#[macro_use] extern crate log;

pub fn from_env_config() -> Result<ClaimConfiguration, JwtCustomError> {
    let toml_str = match fs::read_to_string("./Jwt.toml") {
        Ok(toml) => toml,
        Err(_) => return Err(JwtCustomError::CustomError("Could not get the toml config file".to_string()))
    };
    let config = match toml::from_str(&toml_str) {
        Ok(config) => config,
        Err(e) => return Err(JwtCustomError::CustomError(format!("{}", e)))
    };
    Ok(
        config
    )
}

#[test]
fn test_good_claim() {
    let config = ClaimConfiguration::new("i", "s", 15);
    let claim = config.create_claim("u").unwrap();
    let token = config.token_from_claim(&claim).unwrap();
    assert!(config.decode_token(&token).is_ok());
    assert!(config.decode_token(&token).unwrap().claims.get_subject() == "u");
}

#[test]
fn test_expiration() {
    let config = ClaimConfiguration::new("i", "s", 15);
    let claim = config.create_claim("u");
    assert!(!claim.is_err())
}