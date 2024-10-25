use actix_web::web;
use actix_web::web::ServiceConfig;
use crate::common::traits::controller::Controller;
use crate::item::controller::crud::CrudController;

pub struct Gateway;

impl Controller for Gateway {
    fn cfg(cfg: &mut ServiceConfig) {
        cfg.service(
            web::scope("/item")
                .configure(CrudController::cfg)
        );
    }
}
