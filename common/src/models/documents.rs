use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::DateTimeWithTimeZone;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Docs {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub created: DateTimeWithTimeZone,
    pub last_updated: Option<DateTimeWithTimeZone>,
    pub filetype: Option<String>,
    pub version: i32,
    pub size: i64,
    #[serde(deserialize_with = "deserialize_b64", serialize_with = "serialize_b64")]
    pub data: Option<Vec<u8>>,
}

fn deserialize_b64<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    Ok(Some(raw.into_bytes()))
}

fn serialize_b64<S>(bytes: &Option<Vec<u8>>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(bytes) = bytes {
        let base64 = std::str::from_utf8(bytes).unwrap();
        return s.serialize_some(&base64);
    }
    s.serialize_none()
}

