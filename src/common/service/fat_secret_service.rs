use crate::common::errors::service_error::ServiceError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

pub struct FatSecretService;

#[derive(Debug, Deserialize, Serialize)]
pub struct FatSecretRecipeResponse {
    pub recipes: FatSecretRecipes,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FatSecretRecipes {
    pub max_results: Option<String>,
    pub page_number: Option<String>,
    pub total_results: Option<String>,
    pub recipe: Option<Vec<FatSecretRecipe>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FatSecretRecipe {
    pub recipe_id: Option<String>,
    pub recipe_name: Option<String>,
    pub recipe_description: Option<String>,
    pub recipe_image: Option<String>,
    pub recipe_nutrition: Option<RecipeNutrition>,
    pub recipe_ingredients: Option<RecipeIngredients>,
    pub recipe_types: Option<RecipeTypes>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecipeNutrition {
    pub calories: Option<String>,
    pub carbohydrate: Option<String>,
    pub fat: Option<String>,
    pub protein: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecipeIngredients {
    pub ingredient: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecipeTypes {
    pub recipe_type: Option<Vec<String>>,
}

impl FatSecretService {
    pub async fn get_access_token() -> Result<String, ServiceError> {
        let client = Client::new();
        let token_url = "https://oauth.fatsecret.com/connect/token";

        let client_id = env::var("FATSECRET_CLIENT_ID").map_err(|_| {
            ServiceError::ExternalServiceError("Missing FATSECRET_CLIENT_ID".to_string())
        })?;
        let client_secret = env::var("FATSECRET_CLIENT_SECRET").map_err(|_| {
            ServiceError::ExternalServiceError("Missing FATSECRET_CLIENT_SECRET".to_string())
        })?;
        let grant_type = "client_credentials".to_string();

        let params = [
            ("grant_type", grant_type.as_str()),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
        ];

        let response = client
            .post(token_url)
            .form(&params)
            .send()
            .await
            .map_err(|err| {
                ServiceError::ExternalServiceError(format!("Failed to fetch token: {}", err))
            })?;

        let response_body = response.text().await.map_err(|err| {
            ServiceError::ExternalServiceError(format!("Error reading response: {}", err))
        })?;

        let json: serde_json::Value = serde_json::from_str(&response_body).map_err(|err| {
            ServiceError::ExternalServiceError(format!("JSON parsing failed: {}", err))
        })?;

        if let Some(token) = json["access_token"].as_str() {
            Ok(token.to_string())
        } else {
            Err(ServiceError::ExternalServiceError(
                "Access token not found in response".to_string(),
            ))
        }
    }

    pub async fn get_recipes(
        query: &str,
        access_token: &str,
    ) -> Result<FatSecretRecipeResponse, ServiceError> {
        let client = Client::new();
        let url = "https://platform.fatsecret.com/rest/server.api";

        let response = client
            .get(url)
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&[
                ("method", "recipes.search.v3"),
                ("search_expression", query),
                ("format", "json"),
            ])
            .send()
            .await
            .map_err(|err| {
                ServiceError::ExternalServiceError(format!("FatSecret API request failed: {}", err))
            })?;

        let json: FatSecretRecipeResponse = response.json().await.map_err(|err| {
            ServiceError::ExternalServiceError(format!("JSON parsing failed: {}", err))
        })?;

        Ok(json)
    }
}
