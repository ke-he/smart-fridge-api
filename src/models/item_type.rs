use crate::models::user::User;
use crate::schema::item_type;
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Insertable, Associations)]
#[belongs_to(User, foreign_key = "created_by")]
#[table_name = "item_type"]
pub struct ItemType {
    pub id: i32,
    pub name: String,
    pub created_by: i32,
    pub created_at: NaiveDate,
}
