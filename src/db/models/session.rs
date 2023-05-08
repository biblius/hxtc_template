use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub csrf: Uuid,
    #[serde(with = "ts_datetime")]
    pub created_at: NaiveDateTime,
    #[serde(with = "ts_datetime")]
    pub updated_at: NaiveDateTime,
    #[serde(with = "ts_datetime")]
    pub expires_at: NaiveDateTime,
}

/// Serde utility for serializing `NaiveDateTime`s to timestamps and vice versa.
mod ts_datetime {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};
    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(date.timestamp())
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = i64::deserialize(deserializer)?;
        NaiveDateTime::from_timestamp_millis(millis)
            .ok_or(serde::de::Error::custom("Invalid timestamp"))
    }
}
