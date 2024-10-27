use crate::common::traits::controller::Controller;
use crate::item::service::item_service::ItemService;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse};

pub struct ItemController {
    service: ItemService,
}

impl Controller for ItemController {
    fn cfg(cfg: &mut ServiceConfig) {
        let controller = Self::new();

        cfg.app_data(web::Data::new(controller));

        cfg.service(
            web::scope("")
                .route("/{id}", web::get().to(ItemController::read))
                .route("/body", web::post().to(ItemController::read_body)),
        );
    }
}

#[derive(serde::Deserialize)]
pub struct ReadParam {
    id: i32,
}

impl ItemController {
    pub fn new() -> Self {
        Self {
            service: ItemService,
        }
    }

    pub async fn read(
        controller: web::Data<ItemController>,
        param: web::Path<ReadParam>,
    ) -> HttpResponse {
        let result = controller.service.read();
        Self::response(result + &param.id.to_string())
    }

    pub async fn read_body(
        controller: web::Data<ItemController>,
        body: web::Json<ReadParam>,
    ) -> HttpResponse {
        let result = controller.service.read();
        Self::response(result + &body.id.to_string())
    }
}
