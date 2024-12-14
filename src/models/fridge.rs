use crate::models::user::User;
use crate::schema::fridge;
use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Associations)]
#[belongs_to(User, foreign_key = "created_by")]
#[table_name = "fridge"]
pub struct Fridge {
    pub id: i32,
    pub name: String,
    pub created_by: i32,
    pub created_at: NaiveDate,
}
