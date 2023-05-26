pub mod cache;
pub mod constants;

use crate::app::router;
use crate::error::Error;
use actix_cors::Cors;
use actix_web::http::header::*;
use actix_web::web::ServiceConfig;
use hextacy::drivers::cache::redis::Redis;
use hextacy::drivers::db::postgres::seaorm::PostgresSea;
use std::sync::Arc;

pub type AppResult<T> = Result<T, Error>;

#[derive(Debug, Clone)]
pub struct AppState {
    pub repository: Arc<PostgresSea>,
    pub cache: Arc<Redis>,
}

impl AppState {
    pub async fn init() -> Self {
        Self {
            repository: init_postgres().await,
            cache: init_redis(),
        }
    }
}

pub fn init(state: &AppState, cfg: &mut ServiceConfig) {
    router::route(state, cfg);
}

pub fn setup_cors(allowed_origins: &[&str], expose_headers: &[&str]) -> Cors {
    let mut cors = Cors::default()
        .supports_credentials()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            AUTHORIZATION,
            ACCEPT,
            CONTENT_TYPE,
            ORIGIN,
            ACCESS_CONTROL_REQUEST_METHOD,
            HeaderName::from_static("x-csrf-token"),
        ])
        .expose_headers(expose_headers.to_vec());
    for origin in allowed_origins {
        cors = cors.allowed_origin(origin);
    }
    cors.max_age(3600)
}

async fn init_postgres() -> Arc<PostgresSea> {
    let mut params = hextacy::env::get_multiple(&[
        "PG_USER",
        "PG_PASSWORD",
        "PG_HOST",
        "PG_PORT",
        "PG_DATABASE",
    ]);

    let db = params.pop().expect("PG_DATABASE must be set");
    let port = params.pop().expect("PG_PORT must be set");
    let host = params.pop().expect("PG_HOST must be set");
    let pw = params.pop().expect("PG_PASSWORD must be set");
    let user = params.pop().expect("PG_USER must be set");

    Arc::new(
        PostgresSea::new(
            &host,
            port.parse().expect("Invalid PG_PORT"),
            &user,
            &pw,
            &db,
            8,
        )
        .await,
    )
}

fn init_redis() -> Arc<Redis> {
    let mut params = hextacy::env::get_multiple(&["RD_HOST", "RD_PORT", "RD_DATABASE"]);

    let db = params.pop().expect("RD_DATABASE must be set");
    let port = params.pop().expect("RD_PORT must be set");
    let host = params.pop().expect("RD_HOST must be set");

    Arc::new(Redis::new(
        &host,
        port.parse().expect("Invalid RD_PORT"),
        None,
        None,
        db.parse().expect("Invalid RD_DATABASE"),
        8,
    ))
}
