pub mod adapters;
pub mod contracts;

use hextacy::{cache::CacheError, driver::cache::redis::redis};
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheAdapterError {
    #[error("Hextacy cache error: {0}")]
    Cache(#[from] CacheError),
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
}

/// Implement on cache adapters that should construct keys
pub trait Cacher {
    /// Construct a full cache key using the identifier and key.
    /// Intended to be used by enums that serve as cache identifiers.
    fn key<K: Display>(id: impl KeyPrefix, key: K) -> String {
        format!("{}:{}", id.id(), key)
    }
}

/// Implement on enums that serve as cache keys
pub trait KeyPrefix {
    fn id(self) -> &'static str;
}

/// The domain is the first part of the full key: `domain:id:key`
pub mod domain {
    pub const AUTH: &str = "auth";
}

/// The id is the second part of the full key: `domain:id:key`
pub mod id {
    use super::KeyPrefix;

    #[derive(Debug, PartialEq, Eq)]
    pub enum Auth {
        LoginAttempts,
        Session,
    }

    impl KeyPrefix for Auth {
        fn id(self) -> &'static str {
            use Auth::*;
            match self {
                LoginAttempts => "auth:login_attempts",
                Session => "auth:session",
            }
        }
    }
}
