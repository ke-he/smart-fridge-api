use crate::common::r#type::db_pool::DbPool;
use crate::common::traits::controller::Controller;
use crate::item::controller::structs::items_filter::ItemsFilter;
use crate::item::service::item_service::ItemService;
use crate::models::item::NewItem;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse};

pub struct ItemController;

impl Controller for ItemController {
    fn cfg(cfg: &mut ServiceConfig) {
        let item_service = ItemService;

        cfg.app_data(web::Data::new(item_service));

        cfg.service(
            web::scope("")
                .route("/", web::post().to(ItemController::get_items))
                .route("/add", web::post().to(ItemController::add_item))
                .route("/type", web::get().to(ItemController::get_item_types))
                .route(
                    "/near-expiry",
                    web::get().to(ItemController::get_items_near_expiry),
                )
                .route(
                    "/last-added",
                    web::get().to(ItemController::get_items_last_added),
                ),
        );
    }
}

impl ItemController {
    pub async fn get_items(pool: web::Data<DbPool>, body: web::Json<ItemsFilter>) -> HttpResponse {
        let result = ItemService::get_items(pool.get_ref().clone(), body.into_inner()).await;

        Self::response_handler(result)
    }

    pub async fn add_item(pool: web::Data<DbPool>, body: web::Json<NewItem>) -> HttpResponse {
        let result = ItemService::add_item(pool.get_ref().clone(), body.into_inner()).await;

        Self::response_handler(result)
    }

    pub async fn get_item_types(pool: web::Data<DbPool>) -> HttpResponse {
        let result = ItemService::get_item_types(pool.get_ref().clone()).await;

        Self::response_handler(result)
    }

    pub async fn get_items_near_expiry(pool: web::Data<DbPool>) -> HttpResponse {
        let result = ItemService::get_items_near_expiry(pool.get_ref().clone()).await;

        Self::response_handler(result)
    }

    pub async fn get_items_last_added(pool: web::Data<DbPool>) -> HttpResponse {
        let result = ItemService::get_items_last_added(pool.get_ref().clone()).await;

        Self::response_handler(result)
    }
}
