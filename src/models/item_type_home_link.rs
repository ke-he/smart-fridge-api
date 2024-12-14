use crate::models::home::Home;
use crate::models::item_type::ItemType;
use crate::schema::item_type_home_link;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Associations)]
#[belongs_to(ItemType)]
#[belongs_to(Home)]
#[table_name = "item_type_home_link"]
pub struct ItemTypeHomeLink {
    pub item_type_id: i32,
    pub home_id: i32,
}
