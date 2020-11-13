use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
pub struct EmailHolder {
    email : String
}

impl From<String> for EmailHolder {
    fn from(email : String) -> Self {
        EmailHolder { email : email }
    }
}

impl EmailHolder {
    pub fn get_email(&self) -> &str {
        &self.email
    }
}