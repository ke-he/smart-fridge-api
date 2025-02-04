use crate::common::service::fat_secret_service::FatSecretService;
use crate::common::traits::controller::Controller;
use actix_web::{web, web::ServiceConfig, HttpResponse};
use serde::Deserialize;

pub struct RecipeController;

#[derive(Debug, Deserialize)]
pub struct RecipeQuery {
    pub search_expression: String,
}

impl Controller for RecipeController {
    fn cfg(cfg: &mut ServiceConfig) {
        cfg.service(web::scope("/search").route("", web::get().to(RecipeController::get_recipes)));
    }
}

impl RecipeController {
    pub async fn get_recipes(body: web::Json<RecipeQuery>) -> HttpResponse {
        println!("Value searched for ----------: {}", body.search_expression); // No move occurs

        let access_token: String = match FatSecretService::get_access_token().await {
            Ok(token) => token,
            Err(err) => {
                return HttpResponse::InternalServerError().body(format!("Token Error: {}", err))
            }
        };

        let result = FatSecretService::get_recipes(&body.search_expression, &access_token).await;

        Self::response_handler(result)
    }
}
