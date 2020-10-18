use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
pub struct EmailHolder {
    email : String
}

impl EmailHolder {
    pub fn get_email(&self) -> &str {
        &self.email
    }
}