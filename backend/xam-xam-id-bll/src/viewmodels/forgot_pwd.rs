use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
pub struct ForgottenPassword {
    email : String,
    token : String,
    password : String,
    password_confirm : String
}

impl ForgottenPassword {
    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_password_confirmed(&self) -> &str {
        &self.password_confirm
    }
}