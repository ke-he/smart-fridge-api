use crate::common::param::id_param::IdParam;
use crate::common::r#type::db_pool::DbPool;
use crate::common::traits::controller::Controller;
use crate::item::service::item_service::ItemService;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse};

pub struct ItemController;

impl Controller for ItemController {
    fn cfg(cfg: &mut ServiceConfig) {
        let item_service = ItemService;

        cfg.app_data(web::Data::new(item_service));

        cfg.service(web::scope("").route("/{id}", web::get().to(ItemController::get_item)));
    }
}

impl ItemController {
    pub async fn get_item(pool: web::Data<DbPool>, param: web::Path<IdParam>) -> HttpResponse {
        let result = ItemService::get_item(pool.get_ref().clone(), param.id).await;

        Self::response_handler(result)
    }
}
