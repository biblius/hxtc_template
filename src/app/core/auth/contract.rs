use crate::cache::contracts::SimpleCacheAccess;
use crate::config::AppResult;
use crate::db::models;
use crate::db::repository::{session::SessionRepository, user::UserRepository};
use hextacy::drivers::Connect;
use hextacy::{adapt, contract};

adapt! {
    AuthRepository,

    use Driver for Connection as driver;

    User: UserRepository<Connection>,
    Session: SessionRepository<Connection>
}

#[contract]
impl<D, C, User, Session> AuthRepository<D, C, User, Session>
where
    C: Send,
    D: Connect<Connection = C> + Send + Sync,
    User: UserRepository<C> + Send + Sync,
    Session: SessionRepository<C> + Send + Sync,
{
    async fn get_user_by_email(email: &str) -> AppResult<models::user::User> {
        todo!()
    }
}

adapt! {
    AuthCache,

    use Driver for Connection as driver;

    Cache: SimpleCacheAccess<Connection>
}

#[contract]
impl<D, C, Cache> AuthCache<D, C, Cache>
where
    C: Send,
    D: Connect<Connection = C> + Send + Sync,
    Cache: SimpleCacheAccess<C> + Send + Sync,
{
    async fn get_login_attempts(user_id: &str) -> AppResult<i64> {
        todo!()
    }
}
