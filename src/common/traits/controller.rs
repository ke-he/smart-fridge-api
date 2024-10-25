use actix_web::web::ServiceConfig;

pub trait Controller {
    fn cfg(cfg: &mut ServiceConfig);
}
