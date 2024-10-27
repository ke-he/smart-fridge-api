use crate::models::fridge::Fridge;
use crate::models::item::Item;
use crate::schema::data::*;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Associations)]
#[belongs_to(Item)]
#[belongs_to(Fridge)]
#[table_name = "fridge_item_link"]
pub struct FridgeItemLink {
    pub item_id: i32,
    pub fridge_id: i32,
}
