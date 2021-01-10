use serde::Deserialize;

#[derive(Deserialize)]
pub struct EmailAndPwdHolder {
    email : String,
    pwd : String
}

impl EmailAndPwdHolder {
    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_pwd(&self) -> &str {
        &self.pwd
    }
}