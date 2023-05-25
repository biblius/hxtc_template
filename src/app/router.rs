pub mod resources;

use crate::config::AppState;
use actix_web::web::ServiceConfig;
use hextacy::{route, web::Configure};

use super::setup::auth_middleware::AuthenticationMiddleware;

pub fn route(state: &AppState, cfg: &mut ServiceConfig) {
    let auth_middleware =
        AuthenticationMiddleware::new(state.repository.clone(), state.cache.clone());
    resources::route(cfg);
    auth_service(state, cfg, auth_middleware);
}

pub fn auth_service(state: &AppState, cfg: &mut ServiceConfig, auth_mw: AuthenticationMiddleware) {
    use crate::app::handlers::auth::*;
    use crate::app::setup::auth_service::AuthenticationService;

    AuthenticationService::configure(state, cfg);

    route! {
        AuthenticationService, cfg,
        post => "/auth/register" => register;
        post => "/auth/login" => login;
        post => "/auth/logout" => | auth_mw => logout;
    };
}
