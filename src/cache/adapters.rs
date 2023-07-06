use super::{contracts::SimpleCacheAccess, CacheAdapterError, Cacher, KeyPrefix};
use async_trait::async_trait;
use chrono::Utc;
use hextacy::driver::cache::redis::{redis::AsyncCommands, RedisAdapterExt, RedisConnection};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone)]
pub struct RedisAdapter;

impl RedisAdapterExt for RedisAdapter {}

impl Cacher for RedisAdapter {}

#[async_trait]
impl SimpleCacheAccess<RedisConnection> for RedisAdapter {
    async fn set_str(
        conn: &mut RedisConnection,
        id: impl KeyPrefix + Send,
        key: &str,
        value: &str,
        ex: Option<usize>,
    ) -> Result<(), CacheAdapterError> {
        let key = Self::key(id, key);
        <Self as RedisAdapterExt>::set(conn, key, value, ex)
            .await
            .map_err(CacheAdapterError::Cache)?;
        Ok(())
    }

    async fn get_str(
        conn: &mut RedisConnection,
        id: impl KeyPrefix + Send,
        key: &str,
    ) -> Result<String, CacheAdapterError> {
        let key = Self::key(id, key);
        <Self as RedisAdapterExt>::get(conn, key)
            .await
            .map_err(CacheAdapterError::Cache)
    }

    async fn set_i64(
        conn: &mut RedisConnection,
        id: impl KeyPrefix + Send,
        key: &str,
        value: i64,
        ex: Option<usize>,
    ) -> Result<(), CacheAdapterError> {
        let key = Self::key(id, key);
        Self::set(conn, key, value, ex)
            .await
            .map_err(CacheAdapterError::Cache)?;
        Ok(())
    }

    async fn get_i64(
        conn: &mut RedisConnection,
        id: impl KeyPrefix + Send,
        key: &str,
    ) -> Result<i64, CacheAdapterError> {
        let key = Self::key(id, key);
        <Self as RedisAdapterExt>::get(conn, key)
            .await
            .map_err(CacheAdapterError::Cache)
    }

    async fn get_json<T>(
        conn: &mut RedisConnection,
        id: impl KeyPrefix + Send,
        key: &str,
    ) -> Result<T, CacheAdapterError>
    where
        T: DeserializeOwned,
    {
        let key = Self::key(id, key);
        <Self as RedisAdapterExt>::get_json(conn, key)
            .await
            .map_err(CacheAdapterError::Cache)
    }

    async fn set_json<T>(
        conn: &mut RedisConnection,
        id: impl KeyPrefix + Send,
        key: &str,
        val: T,
        ex: Option<usize>,
    ) -> Result<(), CacheAdapterError>
    where
        T: Serialize + Send + Sync,
    {
        let key = Self::key(id, key);
        <Self as RedisAdapterExt>::set_json(conn, key, val, ex)
            .await
            .map_err(CacheAdapterError::Cache)
    }

    async fn delete(
        conn: &mut RedisConnection,
        id: impl KeyPrefix + Send,
        key: &str,
    ) -> Result<(), CacheAdapterError> {
        let key = Self::key(id, key);
        <Self as RedisAdapterExt>::delete(conn, key)
            .await
            .map_err(CacheAdapterError::Cache)
    }

    async fn refresh(
        conn: &mut RedisConnection,
        id: impl KeyPrefix + Send,
        key: &str,
        duration: i64,
    ) -> Result<(), CacheAdapterError> {
        let key = Self::key(id, key);
        conn.expire_at(key, (Utc::now().timestamp() + duration % i64::MAX) as usize)
            .await
            .map_err(CacheAdapterError::Redis)
    }
}
