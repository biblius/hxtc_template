use crate::cache::adapters::RedisAdapter;
use crate::db::adapters::{session::SessionAdapter, user::UserAdapter};
use crate::AppState;
use actix_web::web::{self, ServiceConfig};
use hextacy::driver::cache::redis::{Redis, RedisConnection};
use hextacy::driver::db::postgres::seaorm::{DatabaseConnection, PostgresSea};
use hextacy::web::Configure;

pub mod auth_middleware {
    use super::*;
    use crate::{
        app::middleware::auth::{
            contract::{AuthMwCache, AuthMwRepo},
            interceptor::AuthenticationGuard,
        },
        db::adapters::session::SessionAdapter,
    };

    type Repository = AuthMwRepo<PostgresSea, DatabaseConnection, SessionAdapter>;
    type Cache = AuthMwCache<Redis, RedisConnection, RedisAdapter>;

    pub type AuthenticationMiddleware = AuthenticationGuard<Repository, Cache>;
}

pub mod auth_service {
    use super::*;
    use crate::app::core::auth::{
        contract::{AuthCache, AuthRepository},
        domain::Authentication,
    };

    type RepositoryComponent =
        AuthRepository<PostgresSea, DatabaseConnection, UserAdapter, SessionAdapter>;

    type CacheComponent = AuthCache<Redis, RedisConnection, RedisAdapter>;

    pub type AuthenticationService = Authentication<RepositoryComponent, CacheComponent>;

    impl Configure<AppState> for AuthenticationService {
        fn configure(state: &AppState, cfg: &mut ServiceConfig) {
            let service = Self {
                repository: RepositoryComponent::new(state.repository.clone()),
                cache: CacheComponent::new(state.cache.clone()),
            };
            cfg.app_data(web::Data::new(service));
        }
    }
}
