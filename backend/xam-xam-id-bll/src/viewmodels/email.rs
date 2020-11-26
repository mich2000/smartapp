use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
pub struct EmailHolder {
    email : String
}

impl From<String> for EmailHolder {
    fn from(new_email : String) -> Self {
        EmailHolder { email : new_email }
    }
}

impl EmailHolder {
    pub fn get_email(&self) -> &str {
        &self.email
    }
}