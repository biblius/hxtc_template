use super::super::entities::users::Entity as UserEntity;
use crate::db::{adapters::AdapterError, models::user::User, repository::user::UserRepository};
use async_trait::async_trait;
use sea_orm::prelude::*;
use sea_orm::ConnectionTrait;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserAdapter;

#[async_trait]
impl<C> UserRepository<C> for UserAdapter
where
    C: ConnectionTrait + Send + Sync,
{
    async fn get_by_id(conn: &mut C, id: &Uuid) -> Result<User, AdapterError> {
        UserEntity::find_by_id(*id)
            .one(conn)
            .await?
            .map_or_else(|| Err(AdapterError::DoesNotExist), |u| Ok(u.into()))
    }
}
