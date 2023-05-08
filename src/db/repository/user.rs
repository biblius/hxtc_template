use crate::db::{adapters::AdapterError, models::user::User};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository<C> {
    async fn get_by_id(conn: &mut C, id: &Uuid) -> Result<User, AdapterError>;
}
