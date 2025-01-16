use crate::common::errors::service_error::ServiceError;
use crate::common::r#type::db_pool::DbPool;
use crate::item::controller::structs::items_filter::ItemsFilter;
use crate::models::item::{Item, NewItem};
use crate::models::item_type::ItemType;
use crate::schema::item::dsl::item;
use crate::schema::item::{created_by, item_type_id, name};
use crate::schema::item_type::dsl::*;
use actix_web::web;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel::QueryDsl;

pub struct ItemService;

impl ItemService {
    const ITEM_EXPIRY_THRESHOLD: i64 = 2;
    const ITEM_LAST_ADDED_LIMIT: i64 = 5;

    pub async fn get_items(
        db_service: DbPool,
        search: ItemsFilter,
    ) -> Result<Vec<Item>, ServiceError> {
        web::block(move || {
            let mut conn = db_service.get().map_err(ServiceError::from)?;

            let mut query = item.into_boxed();

            if let Some(name_filter) = search.name {
                query = query.filter(name.eq(name_filter));
            }
            if let Some(item_type_id_filter) = search.item_type_id {
                query = query.filter(item_type_id.eq(item_type_id_filter));
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
                    created_by.eq(new_item.created_by),
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

    pub async fn get_items_near_expiry(db_service: DbPool) -> Result<Vec<Item>, ServiceError> {
        web::block(move || {
            let mut conn = db_service.get().map_err(ServiceError::from)?;

            let today = Utc::now().naive_utc().date();
            let two_days_later = today + Duration::days(Self::ITEM_EXPIRY_THRESHOLD);

            item.inner_join(crate::schema::item_fridge_link::dsl::item_fridge_link)
                .filter(
                    crate::schema::item_fridge_link::dsl::expiration_date
                        .between(today, two_days_later),
                )
                .order_by(crate::schema::item_fridge_link::dsl::expiration_date.asc())
                .select(crate::schema::item::all_columns)
                .limit(5)
                .load::<Item>(&mut conn)
                .map_err(ServiceError::from)
        })
        .await
        .map_err(|_| ServiceError::BlockingError)?
    }

    pub async fn get_items_last_added(db_service: DbPool) -> Result<Vec<Item>, ServiceError> {
        web::block(move || {
            let mut conn = db_service.get().map_err(ServiceError::from)?;

            item.order_by(crate::schema::item::created_at.desc())
                .limit(Self::ITEM_LAST_ADDED_LIMIT)
                .load::<Item>(&mut conn)
                .map_err(ServiceError::from)
        })
        .await
        .map_err(|_| ServiceError::BlockingError)?
    }
}
