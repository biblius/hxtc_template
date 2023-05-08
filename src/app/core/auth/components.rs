use crate::config::cache;
use crate::db::repository::{session::SessionRepository, user::UserRepository};
use hextacy::cache::CacheAccess;
use hextacy::drivers::cache::redis::Redis;
use hextacy::drivers::Connect;
use hextacy::{adapt, contract};
use std::sync::Arc;

adapt! {
    RepoComponent in super,

    use Driver for Connection as driver;

    User: UserRepository<Connection>,
    Session: SessionRepository<Connection>
}

#[contract]
impl<D, C, User, Session> RepoComponent<D, C, User, Session>
where
    C: Send,
    D: Connect<Connection = C> + Send + Sync,
    User: UserRepository<C> + Send + Sync,
    Session: SessionRepository<C> + Send + Sync,
{
}

pub(super) struct CacheComponent {
    driver: Arc<Redis>,
}

impl CacheComponent {
    pub fn new(driver: Arc<Redis>) -> Self {
        Self { driver }
    }
}

impl CacheAccess for CacheComponent {
    fn domain() -> &'static str {
        cache::domain::AUTH
    }

    fn connection(
        &self,
    ) -> Result<hextacy::drivers::cache::redis::RedisConnection, hextacy::cache::CacheError> {
        self.driver.connect().map_err(|e| e.into())
    }
}

#[contract(super)]
impl CacheComponent {}
