use actix_web::{web::ServiceConfig, HttpResponse};

pub trait Controller {
    fn cfg(cfg: &mut ServiceConfig);

    fn response<T: serde::Serialize>(data: T) -> HttpResponse {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(data)
    }
}
