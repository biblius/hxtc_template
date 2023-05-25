use actix_web::{body::BoxBody, HttpResponseBuilder, Responder};
use hextacy::web::http::response::Response;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use validify::Validify;

#[derive(Debug, Clone, Deserialize, Validify)]
pub struct Register {
    #[modify(trim)]
    #[validate(length(min = 2))]
    pub username: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Validify)]
pub struct Login {
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(length(min = 1))]
    pub password: String,
    pub remember: bool,
}

#[derive(Debug, Deserialize, Validify)]
pub struct Logout {
    pub purge: bool,
}

#[derive(Debug, Serialize)]
pub struct SessionResponse<'a> {
    session_id: &'a str,
}

impl Response<'_> for SessionResponse<'_> {}

impl Responder for SessionResponse<'_> {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        HttpResponseBuilder::new(StatusCode::OK).json(self)
    }
}
