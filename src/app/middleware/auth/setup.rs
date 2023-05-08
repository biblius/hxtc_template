use super::{
    contract::{Cache, Repository},
    interceptor::{AuthenticationGuard, AuthenticationGuardInner},
};
use crate::db::{adapters::session::SessionAdapter, repository::session::SessionRepository};
use hextacy::drivers::{cache::redis::Redis, db::postgres::seaorm::PostgresSea, Connect};
use sea_orm::DatabaseConnection;
use std::{rc::Rc, sync::Arc};

pub type AuthGuardRepoAdapter = Repository<PostgresSea, DatabaseConnection, SessionAdapter>;
pub type AuthenticationMiddleware = AuthenticationGuard<AuthGuardRepoAdapter, Cache>;

impl<D, Conn, Session> AuthenticationGuardInner<Repository<D, Conn, Session>, Cache>
where
    D: Connect<Connection = Conn> + Send + Sync,
    Session: SessionRepository<Conn> + Send + Sync,
{
    pub fn new(adapter: Arc<D>, cache: Arc<Redis>) -> Self {
        Self {
            cache: Cache { driver: cache },
            repository: Repository::new(adapter),
        }
    }
}

impl<A, Conn, Session> AuthenticationGuard<Repository<A, Conn, Session>, Cache>
where
    A: Connect<Connection = Conn> + Send + Sync,
    Session: SessionRepository<Conn> + Send + Sync,
{
    pub fn new(database: Arc<A>, cache: Arc<Redis>) -> Self {
        Self {
            inner: Rc::new(AuthenticationGuardInner::new(database, cache)),
        }
    }
}

/* impl Clone for AuthGuardRepoAdapter {
    fn clone(&self) -> Self {
        Self {
            driver: self.driver.clone(),
            ..*self
        }
    }
} */
