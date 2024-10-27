use crate::models::home::Home;
use crate::models::user::User;
use crate::schema::data::*;
use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Associations)]
#[belongs_to(User)]
#[belongs_to(Home)]
#[table_name = "user_home_link"]
pub struct UserHomeLink {
    pub user_id: i32,
    pub home_id: i32,
    pub join_date: NaiveDate,
    pub is_owner: bool,
}
