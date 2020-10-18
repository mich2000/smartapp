use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
pub struct PasswordTokenHolder {
    token : String
}

impl PasswordTokenHolder {
    pub fn get_token(&self) -> &str {
        &self.token
    }
}