use axum::{routing::get, Router};
use std::net::SocketAddr;
use dotenv::dotenv;
use tracing_subscriber;

mod db;
mod models;
mod auth;
mod handlers;
mod routes;
mod ws;
mod email;

use db::get_db_connection;
use routes::create_router;

async fn health_check() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db = get_db_connection().await;
    tracing::info!("Connected to database");

    
    let app = Router::new()
        .route("/health", get(health_check))
        .merge(create_router(db));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);
    tracing::info!("Customer Support Ticketing System is running!");
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
} 