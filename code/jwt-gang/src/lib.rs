pub mod claim_config;
pub mod claim;
pub mod claim_error;
mod jwt_numeric_date;

#[macro_use] extern crate log;

use claim_config::ClaimConfiguration;
use claim_error::JwtCustomError;

/**
 * Returns a Result with a claim configuration if succeeded, a path is needed.
 */
pub fn from_env_config(path : &str) -> Result<ClaimConfiguration, JwtCustomError> {
    let toml_str = match std::fs::read_to_string(path) {
        Ok(toml) => toml,
        Err(_) => return Err(JwtCustomError::CustomError("Could not get the toml config file for the jwt config".to_string()))
    };
    let config = match toml::from_str(&toml_str) {
        Ok(config) => config,
        Err(e) => return Err(JwtCustomError::CustomError(format!("{}", e)))
    };
    info!("A jwt configuration from the file {} has been created", path);
    Ok(config)
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
fn test_config() {
    let config = from_env_config("Jwt.toml").unwrap();
    assert!(!config.get_issuer().is_empty());
    assert!(!config.get_expiration() != 0);
}