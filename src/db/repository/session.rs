use crate::db::{adapters::AdapterError, models::session::Session};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait SessionRepository<C> {
    async fn get_by_id(conn: &mut C, id: Uuid, csrf: Uuid) -> Result<Session, AdapterError>;
}
