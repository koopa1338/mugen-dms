use sea_orm::entity::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "documents")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub created: Option<DateTimeWithTimeZone>,
    pub last_updated: Option<DateTimeWithTimeZone>,
    pub filetype: String,
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
