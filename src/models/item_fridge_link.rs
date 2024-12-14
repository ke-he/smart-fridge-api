use crate::models::fridge::Fridge;
use crate::models::item::Item;
use crate::schema::item_fridge_link;
use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Associations)]
#[belongs_to(Item)]
#[belongs_to(Fridge)]
#[table_name = "item_fridge_link"]
pub struct ItemFridgeLink {
    pub id: i32,
    pub item_id: i32,
    pub fridge_id: i32,
    pub expiration_date: NaiveDate,
}
