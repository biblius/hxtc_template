mod app;
mod cache;
mod config;
mod db;
mod error;

use crate::config::AppState;
use actix_web::{middleware::Logger, App, HttpServer};
use hextacy::{env, web::http};
use tracing::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let level = std::env::args().nth(1);
    let level = level.as_deref().unwrap_or("debug");

    hextacy::logger::init(level);

    env::load_from_file(".env").unwrap();

    let (host, port) = (
        env::get_or_default("HOST", "127.0.0.1"),
        env::get_or_default("PORT", "3000"),
    );

    info!("Starting with: {:?}", std::env::args());

    let addr = format!("{host}:{port}");
    info!("Starting server on {addr}");

    let state = AppState::init().await;

    HttpServer::new(move || {
        App::new()
            .configure(|cfg| config::init(&state, cfg))
            .wrap(config::setup_cors(&["127.0.0.1"], &["test-header"]))
            .wrap(http::security_headers::default())
            .wrap(Logger::default())
    })
    .bind(addr)?
    .run()
    .await
}
