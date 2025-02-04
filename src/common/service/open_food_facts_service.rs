use crate::common::errors::service_error::ServiceError;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OpenFoodFactsService;

#[derive(Debug, Deserialize, Serialize)]
pub struct NutriscoreData {
    pub score: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Nutriments {
    pub carbohydrates: Option<f32>,
    pub carbohydrates_100g: Option<f32>,
    #[serde(rename = "energy-kcal_100g")]
    pub energy_kcal_100g: Option<f32>,
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
    pub nutriscore_data: Option<NutriscoreData>,
    pub nutriments: Option<Nutriments>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenFoodFactsResponse {
    pub code: String,
    pub product: Option<OpenFoodFactsProduct>,
    pub status: u8,
    pub status_verbose: String,
}

impl OpenFoodFactsService {
    pub async fn get_product(barcode: &str) -> Result<Option<OpenFoodFactsProduct>, ServiceError> {
        let client = Client::new();
        let url = format!(
            "https://world.openfoodfacts.org/api/v2/product/{}?fields=product_name,nutriscore_data,nutriments,nutrition_grades",
            barcode
        );

        let response = client.get(&url).send().await.map_err(|err| {
            ServiceError::ExternalServiceError(format!("API request failed: {}", err))
        })?;

        let api_response: OpenFoodFactsResponse = response.json().await.map_err(|err| {
            ServiceError::ExternalServiceError(format!("JSON parsing failed: {}", err))
        })?;

        if api_response.status == 1 {
            Ok(api_response.product)
        } else {
            Ok(None)
        }
    }

    pub async fn get_product_full(barcode: &str) -> Result<serde_json::Value, ServiceError> {
        let client = Client::new();
        let url = format!("https://world.openfoodfacts.org/api/v2/product/{}", barcode);

        let response = client.get(&url).send().await.map_err(|err| {
            ServiceError::ExternalServiceError(format!("API request failed: {}", err))
        })?;

        let full_json: serde_json::Value = response.json().await.map_err(|err| {
            ServiceError::ExternalServiceError(format!("JSON parsing failed: {}", err))
        })?;

        Ok(full_json)
    }
}
