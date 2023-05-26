use crate::app::core::auth::data::{
    Login, LoginPayload, Logout, LogoutPayload, Register, RegisterPayload,
};
use crate::app::core::auth::domain::AuthenticationContract;
use crate::error::Error;
use actix_web::{web, Responder};
use hextacy::web::http::response::Response;
use reqwest::StatusCode;
use validify::Validify;

pub async fn register<T: AuthenticationContract>(
    data: web::Json<RegisterPayload>,
    service: web::Data<T>,
) -> Result<impl Responder, Error> {
    let registration = Register::validify(data.0).map_err(Error::new)?;
    let session = service.register(registration).await?;
    let res = session
        .to_response(StatusCode::OK)
        .with_cookies(vec![])
        .finish();
    Ok(res)
}

pub async fn login<T: AuthenticationContract>(
    data: web::Json<LoginPayload>,
    service: web::Data<T>,
) -> Result<impl Responder, Error> {
    let credentials = Login::validify(data.0).map_err(Error::new)?;
    let session = service.login(credentials).await?;
    let res = session
        .to_response(StatusCode::OK)
        .with_cookies(vec![])
        .finish();
    Ok(res)
}

pub async fn logout<T: AuthenticationContract>(
    data: web::Json<LogoutPayload>,
    service: web::Data<T>,
) -> Result<impl Responder, Error> {
    let logout = Logout::validify(data.0).map_err(Error::new)?;
    // Message response implements Responder, so we can just return it
    service.logout(logout).await
}
