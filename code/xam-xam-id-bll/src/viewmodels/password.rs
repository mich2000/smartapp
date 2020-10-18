use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
pub struct PasswordHolder {
    password : String,
    password_confirm : String
}

impl PasswordHolder {
    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_password_confirmed(&self) -> &str {
        &self.password_confirm
    }
}