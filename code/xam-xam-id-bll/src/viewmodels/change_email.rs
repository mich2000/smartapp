use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
pub struct EmailsHolder {
    from_email : String,
    to_email : String,
    token : String
}

impl EmailsHolder {
    pub fn get_from_email(&self) -> &str {
        &self.from_email
    }

    pub fn get_to_email(&self) -> &str {
        &self.to_email
    }
    
    pub fn get_token(&self) -> &str {
        &self.token
    }
}