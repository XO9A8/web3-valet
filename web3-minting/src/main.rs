use std::net::SocketAddr;

mod handlers;
mod storage;
mod blockchain;
mod models;

use axum::{routing::post, Router};
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Build our application with a single route
    let app = Router::new().route("/mint", post(handlers::mint));

    // Run on 0.0.0.0:3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!(%addr, "starting web3-minting server");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server failed");
}
use actix_web::{web, App, HttpServer};
mod config;
mod handlers;
mod models;
mod services;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load config
    let settings = config::Settings::from_env().expect("Failed to load settings");
    let settings_data = web::Data::new(settings);

    HttpServer::new(move || {
        App::new()
            .app_data(settings_data.clone())
            .route("/mint", web::post().to(handlers::mint::handle_mint))
            // you may add routes: /mint/status, /assets
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
