use crate::schema::user;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
}
