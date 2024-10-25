mod common;
mod item;

use crate::common::traits::controller::Controller;
use actix_web::{App, HttpServer};

use item::controller::gateway::Gateway as ItemGateway;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(ItemGateway::cfg))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
