use crate::common::controller::recipe_controller::RecipeController;
use crate::common::traits::controller::Controller;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub struct RecipeGateway;

impl Controller for RecipeGateway {
    fn cfg(cfg: &mut ServiceConfig) {
        cfg.service(web::scope("/recipe").configure(RecipeController::cfg));
    }
}
