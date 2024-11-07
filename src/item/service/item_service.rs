use crate::common::errors::service_error::ServiceError;
use crate::common::r#type::db_pool::DbPool;
use crate::item::controller::structs::items_filter::ItemsFilter;
use crate::models::item::{Item, NewItem};
use crate::models::item_type::ItemType;
use crate::schema::data::item::dsl::item;
use crate::schema::data::item::{expiration_date, item_type_id, name};
use crate::schema::data::item_type::dsl::item_type;
use actix_web::web;
use diesel::prelude::*;
use diesel::QueryDsl;

pub struct ItemService;

impl ItemService {
    pub async fn get_items(
        db_service: DbPool,
        search: Option<ItemsFilter>,
    ) -> Result<Vec<Item>, ServiceError> {
        web::block(move || {
            let mut conn = db_service.get().map_err(ServiceError::from)?;

            let mut query = item.into_boxed();

            if let Some(filter) = search {
                if let Some(name_filter) = filter.name {
                    query = query.filter(name.eq(name_filter));
                }
                if let Some(item_type_id_filter) = filter.item_type_id {
                    query = query.filter(item_type_id.eq(item_type_id_filter));
                }
            }

            query.load::<Item>(&mut conn).map_err(ServiceError::from)
        })
        .await
        .map_err(|_| ServiceError::BlockingError)?
    }

    pub async fn add_item(db_service: DbPool, new_item: NewItem) -> Result<usize, ServiceError> {
        web::block(move || {
            let mut conn = db_service.get().map_err(ServiceError::from)?;

            diesel::insert_into(item)
                .values((
                    name.eq(new_item.name),
                    item_type_id.eq(new_item.item_type_id),
                    expiration_date.eq(new_item.expiration_date),
                ))
                .execute(&mut conn)
                .map_err(ServiceError::from)
        })
        .await
        .map_err(|_| ServiceError::BlockingError)?
    }

    pub async fn get_item_types(db_service: DbPool) -> Result<Vec<ItemType>, ServiceError> {
        web::block(move || {
            let mut conn = db_service.get().map_err(ServiceError::from)?;

            item_type
                .load::<ItemType>(&mut conn)
                .map_err(ServiceError::from)
        })
        .await
        .map_err(|_| ServiceError::BlockingError)?
    }
}
