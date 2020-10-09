/**
 * Struct that represents the basic user. This form of user is very simple.
*/
#[derive(Debug,Queryable, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub email : String,
    pub password_hash : String,
    pub salt : String
}