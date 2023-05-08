use super::{components::RepoComponent, handler::*};
use actix_web::web::{self, ServiceConfig};
use hextacy::web::Configure;
use hextacy::{drivers::db::postgres::seaorm::PostgresSea, route};
use sea_orm::DatabaseConnection;

use crate::{
    app::core::auth::components::CacheComponent,
    config::AppState,
    db::adapters::{session::SessionAdapter, user::UserAdapter},
};

use super::service::Authentication;

type RepositoryComponent =
    RepoComponent<PostgresSea, DatabaseConnection, UserAdapter, SessionAdapter>;

type AuthenticationService = Authentication<RepositoryComponent, CacheComponent>;

pub fn routes(state: &AppState, cfg: &mut ServiceConfig) {
    AuthenticationService::configure(state, cfg);

    /*     let auth_guard = AuthenticationGuard::<AuthGuardRepoAdapter, AuthGuardCacheAdapter>::new(
        state.repository.clone(),
        state.cache.clone(),
    ); */

    route! {
        AuthenticationService, cfg,
        post => "/auth/register" => register;
        post => "/auth/login" => login;
        post => "/auth/logout" => logout;
    };
}

impl Configure<AppState> for AuthenticationService {
    fn configure(state: &AppState, cfg: &mut ServiceConfig) {
        let service = Self {
            repository: RepositoryComponent::new(state.repository.clone()),
            cache: CacheComponent::new(state.cache.clone()),
        };
        cfg.app_data(web::Data::new(service));
    }
}
