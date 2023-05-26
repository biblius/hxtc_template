use super::contract::{AuthMwCacheContract, AuthMwRepoContract};
use crate::cache::domain::AUTH;
use crate::config::AppResult;
use crate::db::models::session::Session;
use actix_web::dev::ServiceRequest;
use futures_util::FutureExt;
use hextacy::contract;
use hextacy::{call, transform};
use std::rc::Rc;
use tracing::info;

#[derive(Debug, Clone)]
pub struct AuthenticationGuard<R, C> {
    pub(super) inner: Rc<AuthenticationGuardInner<R, C>>,
}

#[derive(Debug, Clone)]
pub(super) struct AuthenticationGuardInner<R, C> {
    pub repository: R,
    pub cache: C,
}

#[derive(Debug, Clone)]
pub struct AuthenticationGuardMiddleware<S, Repo, Cache> {
    inner: Rc<AuthenticationGuardInner<Repo, Cache>>,
    service: Rc<S>,
}

transform! {
    AuthenticationGuard => AuthenticationGuardMiddleware,
    R: AuthMwRepoContract,
    C: AuthMwCacheContract
}

call! {
    AuthenticationGuardMiddleware,
    R: AuthMwRepoContract,
    C: AuthMwCacheContract;

    fn call(&self, req: ServiceRequest) -> Self::Future {
        info!("Auth guard: Validating session");

        let guard = self.inner.clone();
        let service = self.service.clone();

        async move {
            let res = service.call(req).await?;
            Ok(res)
        }
        .boxed_local()
    }
}

#[contract]
impl<R, C> AuthenticationGuardInner<R, C>
where
    R: AuthMwRepoContract + Send + Sync,
    C: AuthMwCacheContract + Send + Sync,
{
    async fn get_session(id: &str) -> AppResult<Session> {
        todo!()
    }
}
