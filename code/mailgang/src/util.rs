use regex::Regex;

pub fn control_email(email: &str) -> bool {
    Regex::new(r"[a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?").expect("Could not parse the email regex").is_match(email)
}

/**
 * Function used to get a value of the .env config file. If the value isn't found then it searches throught the environmental parameters and if after that it doesn't find it then returns a None.
 */
pub fn get_value_from_key(key : &str) -> Option<String> {
    match dotenv::var(key){
        Ok(value) => Some(value),
        Err(_) => match std::env::var(key) {
            Ok(env_value) => Some(env_value),
            Err(_) => None
        }
    }
}