use serde::{Deserialize, Deserializer, Serialize, Serializer};
use ts_rs::TS;

use crate::DateTimeWithTimeZone;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Docs {
    #[serde(skip_deserializing)]
    pub id: Option<i64>,
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

impl std::fmt::Display for Docs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "Document (")?;
        if let Some(id) = self.id {
            writeln!(f, "\tid: {}", id)?;
        }
        writeln!(f, "\tcreated: {}", self.created)?;
        if let Some(last_updated) = self.last_updated {
            writeln!(f, "\tlast_updated: {}", last_updated)?;
        }
        if let Some(ft) = &self.filetype {
            writeln!(f, "\tfiletype: {}", ft)?;
        }
        writeln!(f, "\tversion: {}", self.version)?;
        writeln!(f, "\tsize: {}", self.size)?;
        write!(f, ")")
    }
}
