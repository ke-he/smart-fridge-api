use crate::models::item_type::ItemType;
use crate::models::user::User;
use crate::schema::item;
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Insertable, Associations)]
#[belongs_to(ItemType)]
#[belongs_to(User, foreign_key = "created_by")]
#[table_name = "item"]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub item_type_id: i32,
    pub code: Option<String>,
    pub created_by: i32,
    pub created_at: NaiveDate,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "item"]
pub struct NewItem {
    pub name: String,
    pub item_type_id: i32,
    pub code: Option<String>,
    pub created_by: i32,
}
