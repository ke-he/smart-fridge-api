mod common;
mod item;
mod models;
mod schema;

use crate::common::r#type::db_pool::DbPool;
use crate::common::traits::controller::Controller;
use crate::item::controller::item_gateway::ItemGateway;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use common::controller::recipe_gateway::RecipeGateway;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;
use std::io::{Error, ErrorKind};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    logger_cfg();

    let pool = database_cfg().await?;

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(web::Data::new(pool.clone()))
            .configure(ItemGateway::cfg)
            .configure(RecipeGateway::cfg)
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
