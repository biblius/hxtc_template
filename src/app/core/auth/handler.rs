use super::data::{Login, LoginPayload, Logout, LogoutPayload, Register, RegisterPayload};
use super::service::AuthenticationContract;
use crate::error::Error;
use actix_web::{web, Responder};
use validify::Validify;

pub(super) async fn register<T: AuthenticationContract>(
    data: web::Json<RegisterPayload>,
    service: web::Data<T>,
) -> Result<impl Responder, Error> {
    let registration = Register::validify(data.0).map_err(Error::new)?;
    service.register(registration).await
}

pub(super) async fn login<T: AuthenticationContract>(
    data: web::Json<LoginPayload>,
    service: web::Data<T>,
) -> Result<impl Responder, Error> {
    let credentials = Login::validify(data.0).map_err(Error::new)?;
    service.login(credentials).await
}

pub(super) async fn logout<T: AuthenticationContract>(
    data: web::Json<LogoutPayload>,
    service: web::Data<T>,
) -> Result<impl Responder, Error> {
    let logout = Logout::validify(data.0).map_err(Error::new)?;
    service.logout(logout).await
}
