use actix_web::{web::ServiceConfig, HttpResponse};

mod response_helpers {
    use actix_web::HttpResponse;
    use serde::Serialize;

    pub(crate) fn response<T: Serialize>(data: T) -> HttpResponse {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(data)
    }

    pub(crate) fn error_response<E: std::fmt::Display>(error: E) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(format!("{{\"error\": \"{}\"}}", error))
    }
}

pub trait Controller {
    fn cfg(cfg: &mut ServiceConfig);

    fn response_handler<T: serde::Serialize, E: std::fmt::Display>(
        result: Result<T, E>,
    ) -> HttpResponse {
        match result {
            Ok(data) => response_helpers::response(data),
            Err(error) => response_helpers::error_response(error),
        }
    }
}
