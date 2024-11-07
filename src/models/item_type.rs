use crate::schema::data::*;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Insertable)]
#[table_name = "item_type"]
pub struct ItemType {
    pub id: i32,
    pub name: String,
}
