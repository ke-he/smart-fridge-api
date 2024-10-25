use crate::common::traits::controller::Controller;
use crate::item::service::crud::CrudService;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse, Responder};

pub struct CrudController {
    service: CrudService,
}

impl Controller for CrudController {
    fn cfg(cfg: &mut ServiceConfig) {
        let controller = Self::new();

        cfg.app_data(web::Data::new(controller));

        cfg.service(web::scope("/crud").route("", web::get().to(CrudController::read)));
    }
}

impl CrudController {
    pub fn new() -> Self {
        Self {
            service: CrudService,
        }
    }

    pub async fn read(controller: web::Data<CrudController>) -> impl Responder {
        let result = controller.service.read();
        HttpResponse::Ok().body(result)
    }
}
