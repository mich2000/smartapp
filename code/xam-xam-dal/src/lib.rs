mod util;
mod enums;

#[macro_use] extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;


/**
 * Have to work on tests again.
 * **/
 #[test]
 fn test_insert() {
     use diesel::prelude::*;
     use diesel::pg::PgConnection;
     use models::user::User;
     use crate::schema::users::dsl::*;

    let conn = PgConnection::establish("postgres://xam-xam-user:xamxambest@localhost/xam-xam").expect("Database connection failed");
    let user = User {
        id : 0,
        email : "michael.hertsens@student.odisee.com".to_string(),
        password_hash : "hash".to_string(),
        salt : "salt".to_string()
    };
    let all_users : Vec<User> = schema::users::table.load(&conn).unwrap();
    if all_users.len() == 0 {
        let result : User = diesel::insert_into(schema::users::table).values(&user).get_result(&conn).unwrap();
        println!("Insert routine");
        let inserted_result : User = schema::users::table.find(0).first(&conn).unwrap();
        assert!(inserted_result.id == 0);
    } else {
        let result : usize = diesel::delete(schema::users::table.filter(id.eq(0))).execute(&conn).unwrap();
        println!("Delete routine");
        assert!(result == 1);
    }
}