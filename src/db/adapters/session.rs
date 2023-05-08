use super::super::entities::sessions::{Column, Entity as SessionEntity};
use crate::db::{
    adapters::AdapterError, models::session::Session, repository::session::SessionRepository,
};
use async_trait::async_trait;
use sea_orm::prelude::*;
use sea_orm::ConnectionTrait;

#[derive(Debug, Clone)]
pub struct SessionAdapter;

#[async_trait]
impl<C> SessionRepository<C> for SessionAdapter
where
    C: ConnectionTrait + Send + Sync,
{
    async fn get_by_id(conn: &mut C, id: &Uuid, csrf: &Uuid) -> Result<Session, AdapterError> {
        SessionEntity::find()
            .filter(Column::Id.eq(*id))
            .filter(Column::Csrf.eq(*csrf))
            .all(conn)
            .await?
            .pop()
            .map_or_else(|| Err(AdapterError::DoesNotExist), |s| Ok(s.into()))
    }
}
