use crate::config::cache;
use crate::db::models::session::{self, Session};
use crate::db::repository::session::SessionRepository;
use crate::error::Error;
use hextacy::drivers::cache::redis::{Redis, RedisConnection};
use hextacy::drivers::Connect;
use hextacy::{adapt, contract};
use hextacy::{cache::CacheAccess, cache::CacheError};
use std::sync::Arc;
use uuid::Uuid;

adapt! {
    Repository,

    use Sea for Connection as driver in super;

    Session: SessionRepository<Connection>
}

#[contract]
impl<D, C, Session> Repository<D, C, Session>
where
    C: Send,
    D: Connect<Connection = C> + Send + Sync,
    Session: SessionRepository<C> + Send + Sync,
{
    async fn get_session(&self, session_id: &Uuid, csrf: &Uuid) -> Result<session::Session, Error> {
        let mut conn = self.driver.connect().await?;
        Session::get_by_id(&mut conn, session_id, csrf)
            .await
            .map_err(Error::new)
    }
}

#[derive(Debug, Clone)]
pub struct Cache {
    pub driver: Arc<Redis>,
}

impl CacheAccess for Cache {
    fn domain() -> &'static str {
        cache::domain::AUTH
    }

    fn connection(&self) -> Result<RedisConnection, CacheError> {
        self.driver.connect().map_err(|e| e.into())
    }
}

#[contract(super)]
impl Cache {
    fn get_session(&self, session_id: &str) -> Result<Session, Error> {
        self.get_json(cache::id::Auth::Session, session_id)
            .map_err(Error::new)
    }
}
