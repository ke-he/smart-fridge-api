use crate::common::errors::service_error::ServiceError;
use crate::common::r#type::db_pool::DbPool;
use crate::models::item::Item;
use crate::schema::data::item::dsl::{id, item};
use actix_web::web;
use diesel::prelude::*;
use diesel::QueryDsl;

pub struct ItemService;

impl ItemService {
    pub async fn get_item(db_service: DbPool, item_id: i32) -> Result<Item, ServiceError> {
        web::block(move || {
            let mut conn = db_service.get().map_err(ServiceError::from)?;
            item.filter(id.eq(item_id))
                .first::<Item>(&mut conn)
                .map_err(ServiceError::from)
        })
        .await
        .map_err(|_| ServiceError::BlockingError)?
    }
}
