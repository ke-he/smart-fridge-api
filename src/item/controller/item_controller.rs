use crate::common::param::id_param::IdParam;
use crate::common::traits::controller::Controller;
use crate::item::service::item_service::ItemService;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

pub struct ItemController;

impl Controller for ItemController {
    fn cfg(cfg: &mut ServiceConfig) {
        let item_service = ItemService;

        cfg.app_data(web::Data::new(item_service));

        cfg.service(web::scope("").route("/{id}", web::get().to(ItemController::get_item)));
    }
}

impl ItemController {
    pub async fn get_item(
        pool: web::Data<PgPool>,
        item_service: web::Data<ItemService>,
        param: web::Path<IdParam>,
    ) -> HttpResponse {
        let result = item_service.get_item(&pool, &param.id).await;
        Self::response(result)
    }
}
