use crate::common::errors::service_error::ServiceError;
use crate::common::r#type::db_pool::DbPool;
use crate::schema::data::item::dsl::{id, item, name};
use actix_web::web;
use diesel::prelude::*;
use diesel::QueryDsl;

pub struct ItemService;

impl ItemService {
    pub async fn get_item(db_service: DbPool, item_id: i32) -> Result<String, ServiceError> {
        web::block(move || {
            let mut conn = db_service.get().map_err(ServiceError::from)?;
            item.filter(id.eq(item_id))
                .select(name)
                .first::<String>(&mut conn)
                .map_err(ServiceError::from)
        })
        .await
        .map_err(|_| ServiceError::BlockingError)?
    }
}
