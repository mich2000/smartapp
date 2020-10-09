mod util;
mod enums;

#[macro_use] extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use models::user::User;

/**
 * Have to work on tests again.
 * **/
#[test]
fn test_insert() {
    use diesel::prelude::*;
    use diesel::pg::PgConnection;

    let conn = PgConnection::establish("postgres://xam-xam-user:xamxambest@localhost/xam-xam").expect("Database connection failed");
    let user = User {
        id : 0,
        email : "michael.hertsens@student.odisee.com".to_string(),
        password_hash : "hash".to_string(),
        salt : "salt".to_string()
    };
    //let result : User = diesel::insert_into(schema::users::table).values(&user).get_result(&conn).unwrap();
    let user_db : Vec<User> = schema::users::table.load(&conn).unwrap();
    assert!(user_db.len() == 1)
}