use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
pub struct TokenHolder {
    token : String
}

impl TokenHolder {
    pub fn get_token(&self) -> &str {
        &self.token
    }
}