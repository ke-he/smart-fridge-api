use crate::models::fridge::Fridge;
use crate::models::home::Home;
use crate::schema::data::*;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Associations)]
#[belongs_to(Home)]
#[belongs_to(Fridge)]
#[table_name = "home_fridge_link"]
pub struct HomeFridgeLink {
    pub home_id: i32,
    pub fridge_id: i32,
}
