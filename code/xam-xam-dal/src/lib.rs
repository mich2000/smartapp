mod util;
mod enums;

#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;
extern crate dotenv;

pub mod schema;
pub mod models;
pub mod err;

/**
 * Have to work on tests again.
 * **/
 #[test]
 fn test_insert() {
     use diesel::prelude::*;
     use diesel::pg::PgConnection;
     use models::user::User;
     use crate::schema::users::dsl::*;
     use models::user::InsertableUser;

    let conn = PgConnection::establish("postgres://xam-xam-user:xamxambest@localhost/xam-xam").expect("Database connection failed");
    let mut all_users : Vec<User> = schema::users::table.load(&conn).unwrap();
    if all_users.len() == 0 {
        let insertion = InsertableUser::new("michael.hertsens@student.odisee.com","hash");
        if insertion.is_ok() {
            let result : User = diesel::insert_into(schema::users::table).values(&insertion.unwrap()).get_result(&conn).unwrap();
            println!("Insert routine");
            schema::users::table.find(0).first::<User>(&conn).optional();
            all_users = schema::users::table.load(&conn).unwrap();
            assert!(all_users.len() == 1);
        } else {
            println!("{:#?}", insertion);
        }
    } else {
        let result : usize = diesel::delete(schema::users::table.filter(email.eq("michael.hertsens@student.odisee.com"))).execute(&conn).unwrap();
        println!("Delete routine");
        assert!(result == 1);
    }
}