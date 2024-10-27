use crate::schema::data::*;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[table_name = "home"]
pub struct Home {
    pub id: i32,
    pub name: String,
}
