mod common;
mod item;
mod models;
mod schema;

use crate::common::r#type::db_pool::DbPool;
use crate::common::traits::controller::Controller;
use crate::item::controller::item_gateway::ItemGateway;
use crate::item::service::item_service::ItemService;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;
use std::io::{Error, ErrorKind};

// Route: API-Endpunkt zum Testen der OpenFoodFacts-API

// Beispiel für eine Funktion, die JSON zurückgibt
#[get("/test-openfood/{barcode}")]
async fn test_openfood(barcode: web::Path<String>) -> impl Responder {
    let barcode = barcode.into_inner(); // Barcode aus der URL extrahieren
    match ItemService::get_openfood_product(&barcode).await {
        Ok(Some(product)) => HttpResponse::Ok().json(product), // Produkt als JSON zurückgeben
        Ok(None) => HttpResponse::NotFound().body("Product not found"),
        Err(err) => HttpResponse::InternalServerError().body(format!("API error: {}", err)),
    }
}

#[get("/openfood/full/{barcode}")]
async fn full_openfood(barcode: web::Path<String>) -> impl Responder {
    let barcode = barcode.into_inner();

    // Rufe die Funktion auf, die den vollständigen JSON-Response zurückgibt
    match ItemService::get_openfood_product_full(&barcode).await {
        Ok(full_json) => HttpResponse::Ok().json(full_json),
        Err(err) => HttpResponse::InternalServerError().body(format!("API error: {}", err)),
    }
}

#[get("/fatsecret/recipes/{query}")]
async fn fatsecret_recipes(query: web::Path<String>) -> impl Responder {
    let query = query.into_inner();

    match ItemService::get_fatsecret_access_token().await {
        Ok(access_token) => {
            match ItemService::get_fatsecret_recipes(&query, &access_token).await {
                Ok(recipes) => HttpResponse::Ok().json(recipes), // Return the recipes as JSON
                Err(err) => HttpResponse::InternalServerError().body(format!("API error: {}", err)),
            }
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Error getting access token: {}", err))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    logger_cfg();

    let pool = database_cfg().await?;

    match ItemService::get_fatsecret_access_token().await {
        Ok(token) => println!("✅ FatSecret Access Token: {}", token),
        Err(err) => println!("❌ Error getting access token: {}", err),
    }

    // 🔍 Test for chicken, aber geht nicht gerade
    match ItemService::get_fatsecret_recipes("chicken", "your_temp_token_here").await {
        Ok(recipes) => println!("✅ FatSecret Recipes Response: {:?}", recipes),
        Err(err) => println!("❌ Error fetching recipes: {}", err),
    }

    // Testaufruf für die OpenFoodFacts-API (nur für die Logs)
    let barcode = "3017624010701";
    match ItemService::get_openfood_product(barcode).await {
        Ok(Some(product)) => {
            println!("Product found for barcode {}: {:?}", barcode, product);
        }
        Ok(None) => {
            println!("No product found for barcode {}", barcode);
        }
        Err(err) => {
            println!("Error getting product for barcode {}: {:?}", barcode, err);
        }
    }

    // Das Auskommentieren damit weniger shit im Log steht
    /*match ItemService::get_openfood_product_full("3017624010701").await {
        Ok(json) => println!("Full JSON response: {}", json),
        Err(err) => println!("Error: {}", err),
    }*/

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(test_openfood)
            .service(full_openfood)
            .service(fatsecret_recipes)
            .app_data(web::Data::new(pool.clone()))
            .configure(ItemGateway::cfg)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn logger_cfg() {
    let rust_log: String = env::var("RUST_LOG").unwrap_or("info".to_string());
    let rust_backtrace: String = env::var("RUST_BACKTRACE").unwrap_or("1".to_string());

    env::set_var("RUST_LOG", rust_log);
    env::set_var("RUST_BACKTRACE", rust_backtrace);

    env_logger::init();
}

async fn database_cfg() -> Result<DbPool, Error> {
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| Error::new(ErrorKind::NotFound, "DATABASE_URL not set"))?;
    let max_connections: u32 = env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "10".to_string()) // default to 10 connections if unset
        .parse()
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid max connections"))?;

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(max_connections)
        .build(manager)
        .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to create pool: {}", e)))?;

    Ok(pool)
}
