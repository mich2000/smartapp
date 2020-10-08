pub struct ClaimConfiguration {
    claim_issuer : String,
    claim_secret : String,
    claim_expiration : i64
}

impl ClaimConfiguration {
    pub fn new(issuer : &str, secret : &str, expiration : i64) -> Self {
        Self {
            claim_issuer : issuer.to_string(),
            claim_secret : secret.to_string(),
            claim_expiration : expiration
        }
    }
}