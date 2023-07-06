use crate::cache::contracts::SimpleCacheAccess;
use crate::db::models::session::{self, Session};
use crate::db::repository::session::SessionRepository;
use crate::error::Error;
use hextacy::driver::Driver;
use hextacy::{contract, drive};
use uuid::Uuid;

drive! {
    AuthMwRepo,
    use Driver for C as driver in super;
    Session: SessionRepository<C>
}

#[contract]
impl<D, C, Session> AuthMwRepo<D, C, Session>
where
    C: Send,
    D: Driver<Connection = C> + Send + Sync,
    Session: SessionRepository<C> + Send + Sync,
{
    async fn get_session(&self, session_id: Uuid, csrf: Uuid) -> Result<session::Session, Error> {
        let mut conn = self.driver.connect().await?;
        Session::get_by_id(&mut conn, session_id, csrf)
            .await
            .map_err(Error::new)
    }
}

drive! {
    AuthMwCache,
    use Driver for Connection as driver;
    Cache: SimpleCacheAccess<Connection>
}

#[contract(super)]
#[contract]
impl<D, C, Cache> AuthMwCache<D, C, Cache>
where
    C: Send,
    D: Driver<Connection = C> + Send + Sync,
    Cache: SimpleCacheAccess<C> + Send + Sync,
{
    async fn get_session(&self, session_id: &str) -> Result<Session, Error> {
        let mut conn = self.driver.connect().await?;
        Cache::get_json(&mut conn, crate::cache::id::Auth::Session, session_id)
            .await
            .map_err(Error::new)
    }
}

impl<D, C, S> Clone for AuthMwRepo<D, C, S>
where
    D: Driver<Connection = C> + Send + Sync,
    S: SessionRepository<C> + Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            driver: self.driver.clone(),
            ..*self
        }
    }
}

impl<D, C, Cache> Clone for AuthMwCache<D, C, Cache>
where
    D: Driver<Connection = C> + Send + Sync,
    Cache: SimpleCacheAccess<C> + Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            driver: self.driver.clone(),
            ..*self
        }
    }
}
