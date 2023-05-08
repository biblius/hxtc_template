use actix_web::{body::BoxBody, HttpResponseBuilder, Responder};
use reqwest::{header, StatusCode};
use std::fs;
use tracing::debug;

lazy_static::lazy_static! {
    pub static ref FAVICON: Vec<u8> = {
        debug!("Loading favicon");
        fs::read("resources/favicon.ico").expect("Couldn't load favicon.ico")
    };
}

pub async fn favicon() -> impl Responder {
    HttpResponseBuilder::new(StatusCode::OK)
        .append_header((
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("image/x-ico"),
        ))
        .body(BoxBody::new(FAVICON.clone()))
}
