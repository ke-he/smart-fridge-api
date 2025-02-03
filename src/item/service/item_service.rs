use std::env;
use crate::common::errors::service_error::ServiceError;
use crate::common::r#type::db_pool::DbPool;
use crate::item::controller::structs::items_filter::ItemsFilter;
use crate::models::item::{Item, NewItem};
use crate::models::item_type::ItemType;
use crate::schema::item::dsl::item;
use crate::schema::item::{created_by, item_type_id, name};
use crate::schema::item_type::dsl::*;
use actix_web::web;
use diesel::prelude::*;
use diesel::QueryDsl;
// use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct ItemService;

#[derive(Debug, Deserialize, Serialize)]
pub struct NutriscoreData {
    pub score: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Nutriments {
    pub carbohydrates: Option<f32>,
    pub carbohydrates_100g: Option<f32>,
    #[serde(rename = "energy-kcal_100g")]
    pub energy_kcal_100g: Option<f32>, // JSON-Feld "energy-kcal_100g"
    pub fat: Option<f32>,
    pub fat_100g: Option<f32>,
    pub proteins: Option<f32>,
    pub proteins_100g: Option<f32>,
    pub sugars: Option<f32>,
    pub sugars_100g: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenFoodFactsProduct {
    pub nutrition_grades: Option<String>,
    pub product_name: Option<String>,
    pub nutriscore_data: Option<NutriscoreData>, // Mapping von nutriscore_data
    pub nutriments: Option<Nutriments>,          //  nutriments
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenFoodFactsResponse {
    pub code: String,
    pub product: Option<OpenFoodFactsProduct>,
    pub status: u8,
    pub status_verbose: String,
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

impl ItemService {
    pub async fn get_items(
        db_service: DbPool,
        search: ItemsFilter,
    ) -> Result<Vec<Item>, ServiceError> {
        web::block(move || {
            let mut conn = db_service.get().map_err(ServiceError::from)?;

            let mut query = item.into_boxed();

            if let Some(name_filter) = search.name {
                query = query.filter(name.eq(name_filter));
            }
            if let Some(item_type_id_filter) = search.item_type_id {
                query = query.filter(item_type_id.eq(item_type_id_filter));
            }

            query.load::<Item>(&mut conn).map_err(ServiceError::from)
        })
        .await
        .map_err(|_| ServiceError::BlockingError)?
    }

    pub async fn add_item(db_service: DbPool, new_item: NewItem) -> Result<usize, ServiceError> {
        web::block(move || {
            let mut conn = db_service.get().map_err(ServiceError::from)?;

            diesel::insert_into(item)
                .values((
                    name.eq(new_item.name),
                    item_type_id.eq(new_item.item_type_id),
                    created_by.eq(new_item.created_by),
                ))
                .execute(&mut conn)
                .map_err(ServiceError::from)
        })
        .await
        .map_err(|_| ServiceError::BlockingError)?
    }

    pub async fn get_item_types(db_service: DbPool) -> Result<Vec<ItemType>, ServiceError> {
        web::block(move || {
            let mut conn = db_service.get().map_err(ServiceError::from)?;

            item_type
                .load::<ItemType>(&mut conn)
                .map_err(ServiceError::from)
        })
        .await
        .map_err(|_| ServiceError::BlockingError)?
    }

    pub async fn get_openfood_product(
        barcode: &str,
    ) -> Result<Option<OpenFoodFactsProduct>, ServiceError> {
        let client = reqwest::Client::new();
        let url = format!(
            "https://world.openfoodfacts.org/api/v2/product/{}?fields=product_name,nutriscore_data,nutriments,nutrition_grades",
            barcode
        );

        // API-Aufruf
        let response = client.get(&url).send().await.map_err(|err| {
            ServiceError::ExternalServiceError(format!("API request failed: {}", err))
        })?;

        // JSON-Daten umwandeln
        let api_response: OpenFoodFactsResponse = response.json().await.map_err(|err| {
            ServiceError::ExternalServiceError(format!("JSON parsing failed: {}", err))
        })?;

        // Produkt zurückgeben, falls gefunden
        if api_response.status == 1 {
            Ok(api_response.product)
        } else {
            Ok(None) // Kein Produkt gefunden
        }
    }

    pub async fn get_openfood_product_full(
        barcode: &str,
    ) -> Result<serde_json::Value, ServiceError> {
        let client = reqwest::Client::builder()
            .user_agent("MyApp - Version 1.0 - www.myapp.com")
            .build()
            .map_err(|err| {
                ServiceError::ExternalServiceError(format!("Failed to create client: {}", err))
            })?;

        let url = format!("https://world.openfoodfacts.org/api/v2/product/{}", barcode);

        // API-Aufruf
        let response = client.get(&url).send().await.map_err(|err| {
            ServiceError::ExternalServiceError(format!("API request failed: {}", err))
        })?;

        // Komplette JSON-Antwort
        let full_json: serde_json::Value = response.json().await.map_err(|err| {
            ServiceError::ExternalServiceError(format!("JSON parsing failed: {}", err))
        })?;

        Ok(full_json)
    }

    pub async fn get_fatsecret_access_token() -> Result<String, ServiceError> {
        let client = reqwest::Client::new();
        let token_url = "https://oauth.fatsecret.com/connect/token";

        // guck fit guardian

        let client_id = env::var("FATSECRET_CLIENT_ID")
            .map_err(|_| ServiceError::ExternalServiceError("Missing FATSECRET_CLIENT_ID".to_string()))?;
        let client_secret = env::var("FATSECRET_CLIENT_SECRET")
            .map_err(|_| ServiceError::ExternalServiceError("Missing FATSECRET_CLIENT_SECRET".to_string()))?;
        let grant_type = env::var("FATSECRET_GRANT_TYPE")
            .unwrap_or("client_credentials".to_string());

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

        // Weirder fehler. Chatgpt logik xD
        let response_body = response.text().await.map_err(|err| {
            ServiceError::ExternalServiceError(format!("Error reading response: {}", err))
        })?;

        // Raw Response ausgeben
        println!("🔍 FatSecret Raw Response: {}", response_body);

        // Parse JSON, aber kein JSON  bis jetzt
        let json: serde_json::Value = serde_json::from_str(&response_body).map_err(|err| {
            ServiceError::ExternalServiceError(format!("JSON parsing failed: {}", err))
        })?;

        // Token holen
        if let Some(token) = json["access_token"].as_str() {
            println!("✅ Successfully retrieved access token: {}", token); // Debugging
            Ok(token.to_string())
        } else {
            Err(ServiceError::ExternalServiceError(
                "Access token not found in response".to_string(),
            ))
        }
    }

    pub async fn get_fatsecret_recipes(
        query: &str,
        access_token: &str,
    ) -> Result<FatSecretRecipeResponse, ServiceError> {
        let client = reqwest::Client::new();
        let url = "https://platform.fatsecret.com/rest/server.api";

        // ✅ Bearer Token muss header sein
        let response = client
            .get(url)
            .header("Authorization", format!("Bearer {}", access_token)) // das ist der header
            .query(&[
                ("method", "recipes.search.v3"),
                ("search_expression", query),
                ("format", "json"),
            ])
            .send()
            .await
            .map_err(|err| {
                ServiceError::ExternalServiceError(format!(
                    "❌ FatSecret API request failed: {}",
                    err
                ))
            })?;

        let raw_response = response.text().await.map_err(|err| {
            ServiceError::ExternalServiceError(format!(
                "❌ Error reading FatSecret response: {}",
                err
            ))
        })?;

        // Raw api
        println!("📝 Raw FatSecret API Response: {}", raw_response);

        // ✅ Debugging
        Err(ServiceError::ExternalServiceError(format!(
            "Raw API Response: {}",
            raw_response
        )))
    }
}
