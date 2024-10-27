use crate::common::traits::controller::Controller;
use crate::item::controller::item_controller::ItemController;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub struct ItemGateway;

impl Controller for ItemGateway {
    fn cfg(cfg: &mut ServiceConfig) {
        cfg.service(web::scope("/item").configure(ItemController::cfg));
    }
}
