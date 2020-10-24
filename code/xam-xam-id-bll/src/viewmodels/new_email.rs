use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
pub struct NewEmailHolder {
    token : String,
    email : String
}

impl NewEmailHolder {
    pub fn get_token(&self) -> &str {
        &self.token
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }
}