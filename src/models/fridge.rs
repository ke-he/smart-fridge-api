use crate::schema::data::*;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[table_name = "fridge"]
pub struct Fridge {
    pub id: i32,
    pub name: String,
}
