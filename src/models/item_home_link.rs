use crate::models::home::Home;
use crate::models::item::Item;
use crate::schema::data::*;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Associations)]
#[belongs_to(Item)]
#[belongs_to(Home)]
#[table_name = "item_home_link"]
pub struct ItemHomeLink {
    pub item_id: i32,
    pub home_id: i32,
}
