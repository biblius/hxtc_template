pub mod contract;
pub mod interceptor;

use crate::{cache::contracts::SimpleCacheAccess, db::repository::session::SessionRepository};
use hextacy::drivers::Connect;
use std::{rc::Rc, sync::Arc};

use self::{
    contract::{AuthMwCache, AuthMwRepo},
    interceptor::{AuthenticationGuard, AuthenticationGuardInner},
};

impl<RepoDriver, CacheDriver, RepoConn, CacheConn, Session, Cache>
    AuthenticationGuardInner<
        AuthMwRepo<RepoDriver, RepoConn, Session>,
        AuthMwCache<CacheDriver, CacheConn, Cache>,
    >
where
    RepoDriver: Connect<Connection = RepoConn> + Send + Sync,
    Session: SessionRepository<RepoConn> + Send + Sync,
    CacheDriver: Connect<Connection = CacheConn> + Send + Sync,
    Cache: SimpleCacheAccess<CacheConn> + Send + Sync,
{
    pub fn new(adapter: Arc<RepoDriver>, cache: Arc<CacheDriver>) -> Self {
        Self {
            cache: AuthMwCache::new(cache),
            repository: AuthMwRepo::new(adapter),
        }
    }
}

impl<RepoDriver, CacheDriver, CacheConn, Cache, RepoConn, Session>
    AuthenticationGuard<
        AuthMwRepo<RepoDriver, RepoConn, Session>,
        AuthMwCache<CacheDriver, CacheConn, Cache>,
    >
where
    CacheDriver: Connect<Connection = CacheConn> + Send + Sync,
    RepoDriver: Connect<Connection = RepoConn> + Send + Sync,
    Cache: SimpleCacheAccess<CacheConn> + Send + Sync,
    Session: SessionRepository<RepoConn> + Send + Sync,
{
    pub fn new(repository: Arc<RepoDriver>, cache: Arc<CacheDriver>) -> Self {
        Self {
            inner: Rc::new(AuthenticationGuardInner::new(repository, cache)),
        }
    }
}

#[cfg(test)]
mod tests {}
