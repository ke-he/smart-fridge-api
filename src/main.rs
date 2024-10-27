mod common;
mod item;

use crate::common::traits::controller::Controller;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use item::controller::item_gateway::ItemGateway;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let rust_log: String = env::var("RUST_LOG").unwrap_or("info".to_string());
    let rust_backtrace: String = env::var("RUST_BACKTRACE").unwrap_or("1".to_string());

    env::set_var("RUST_LOG", rust_log);
    env::set_var("RUST_BACKTRACE", rust_backtrace);

    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).configure(ItemGateway::cfg)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
