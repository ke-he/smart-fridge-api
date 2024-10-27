mod common;
mod item;

use crate::common::traits::controller::Controller;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use item::controller::item_gateway::ItemGateway;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
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

async fn database_cfg() -> Result<PgPool, Error> {
    let database_url: String = env::var("DATABASE_URL").unwrap_or("".to_string());
    let database_max_connections: u32 = env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or("0".to_string())
        .parse()
        .unwrap_or(0);

    if database_url.is_empty() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "DATABASE_URL environment variable is empty.",
        ));
    }

    if database_max_connections == 0 {
        return Err(Error::new(
            ErrorKind::NotFound,
            "DATABASE_MAX_CONNECTIONS environment variable is empty.",
        ));
    }

    let pool = PgPoolOptions::new()
        .max_connections(database_max_connections)
        .connect(&database_url)
        .await
        .map_err(|error| {
            Error::new(
                ErrorKind::Other,
                format!(
                    "Failed to connect to the database (url: {}): {}",
                    database_url, error
                ),
            )
        })?;

    Ok(pool)
}
