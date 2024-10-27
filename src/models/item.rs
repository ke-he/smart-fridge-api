use crate::models::item_type::ItemType;
use crate::schema::data::*;
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Insertable, Associations)]
#[belongs_to(ItemType)]
#[table_name = "item"]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub item_type_id: i32,
    pub expiration_date: NaiveDate,
}
