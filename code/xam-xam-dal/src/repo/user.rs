use diesel::PgConnection;

pub fn new_user(email : &str, password : &str)